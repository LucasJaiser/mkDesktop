
use std::path::Path;
use std::env;

use clap::Parser;
use requestty::Question;
use serde::{Deserialize, Serialize};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use requestty::question::{completions, Completions};

use crate::info::AppInfo;
use crate::info::AppType;

mod info;
mod detector;

//Paths for where the actuall .desktop files will go
static GLOBAL_PATH: &str = "/usr/share/applications";
static LOCAL_PATH: &str = "~/.local/share/applications";

#[derive(Parser)]
#[clap(author="Lucas Jaiser", version="1.2.1", about, long_about = "A CLI tool to create .desktop files with ease")]
struct Cli {
    /// Name of the File you want to create
    #[clap(short, long)]
    name: Option<String>,

    ///Application Type (possible values: Application, Link, Directory). 
    #[clap(short, long, default_value_t = String::from("Application") )]
    app_type: String,

    ///Categories wich describes your Application, you can find possible Categories here: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
    #[clap(short, long, default_value_t = String::from("Utility"))]
    categories: String,

    ///The binary or .sh etc. which should be executed
    #[clap(short, long)]
    exec: Option<String>,
    
    ///The Icon wich will be displayed with this Application
    #[clap(short, long)]
    icon: Option<String>,
    
    ///Should mkDesktop install the in global Directory or in the Local only for the current user
    #[clap(short, long, default_value_t = String::from("global"))]
    global: String,

    ///Starts the guided mode of mkDesktop, you will get asked step by step all needed Information.
    ///Good for beginners 
    #[clap(short = 'G', long)]
    guided: bool,

    ///Only Print out a template of the .desktop file. 
    #[clap(short, long)]
    template: bool,

    ///Auto detect informations 
    ///Only the following field can be detected: name (folder name), exec (file rights), icon (filename), global is predefined to "global", app-type is predefined to "Application"
    #[clap(short = 'A', long)]
    auto_detect: bool,
    
    ///Path in which the file will be written. Warning: Overwrites the global and local Path(if the
    ///global parameter is set it will be no longer active. The app will use this Path). 
    #[clap(short = 'p', long)]
    path: Option<String>,

    ///Redirects output of the file to stdout
    #[clap(short = 'o', long)]
    output: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config{
    global_path: String,
    local_path: String,
    default_categorie: String,
    default_app_type: String,
}
  
fn main() {
    let cli = Cli::parse();
    let cfg: Config = confy::load("mkDesktop").unwrap();
    let mut info: AppInfo;
    let mut output_path: String;
    let categorie: String;
    let app_type: String;

    //Template Mode
    if cli.template {
        AppInfo::print_template();
        return;
    }
    
    //Set default value for categories
    if cfg.default_categorie != "" {
        categorie = cfg.default_categorie.clone();
    }else{
        categorie = cli.categories.clone();
    }

    //Set dault value for app_type
    if cfg.default_app_type != "" {
        app_type = cfg.default_app_type.clone();
    }else{
        app_type = cli.app_type.clone();
    }

    //Auto Detection Mode
    if cli.auto_detect {
        
        let info_return = detector::detect(env::current_dir().unwrap());
        match info_return {
            Ok(info_return) => {    
                info = info_return;
                info.categories = categorie;
                info.application_type = AppType::convert_app_type(&app_type).unwrap();
            },
            Err(..) => {return;},
        }
    //cli Mode
    }else{

        if cli.guided ||cli.name.is_none() {
            println!("---------------Guided Mode----------------");
            //Start guided Input mode, this is where the information to the .Desktop file is gathered.
            info = guided_input();
        }else{

            //Check for missing information
            if cli.exec.is_none() {
                println!("Information to create the file not Provided: exec. Add --exec 'executable' to the command to fix.");
                return;
            }
            
            info = AppInfo::new(cli.name.unwrap(), cli.exec.unwrap(), categorie, AppType::convert_app_type(&app_type).unwrap(), cli.icon.unwrap(), cli.global);
        }
    }
  
    //load path Variable from config or predefined Path, check if user wants a global or local isntallation
    if info.global.eq("global") {
        if cfg.global_path != "" {
            output_path = cfg.global_path.clone();
        }else{
            output_path = GLOBAL_PATH.to_string(); 
        }
    }else{
        if cfg.local_path != "" {
            output_path = cfg.local_path.clone();
        }else{
            output_path = LOCAL_PATH.to_string();
        }
    }

    if cli.path.is_some() {
        output_path = cli.path.unwrap();
    }
    
       //takes the struct and writes it to the actual file in the correct Location based on input 
    AppInfo::write_info_to_file(info, output_path, cli.output);
        
    confy::store("mkDesktop", cfg).unwrap();

}

fn auto_complete(p: String) -> Completions<String> {
    let current: &Path = p.as_ref();
    let (mut dir, last) = if p.ends_with('/') {
        (current, "")
    } else {
        let dir = current.parent().unwrap_or_else(|| "/".as_ref());
        let last = current
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");
        (dir, last)
    };

    if dir.to_str().unwrap().is_empty() {
        dir = ".".as_ref();
    }

    let mut files: Completions<_> = match dir.read_dir() {
        Ok(files) => files
            .flatten()
            .flat_map(|file| {
                let path = file.path();
                let is_dir = path.is_dir();
                match path.into_os_string().into_string() {
                    Ok(s) if is_dir => Some(s + "/"),
                    Ok(s) => Some(s),
                    Err(_) => None,
                }
            })
            .collect(),
        Err(_) => {
            return completions![p];
        }
    };

    if files.is_empty() {
        return completions![p];
    } else {
        let fuzzer = SkimMatcherV2::default();
        files.sort_by_cached_key(|file| fuzzer.fuzzy_match(file, last).unwrap_or(i64::MAX));
        files
    }
}


///This function gathers information to the .Desktop file in a Guided form. 
///it leads you through all field you will need in the file to be valid. 
///Technically you only need Type, Name and Exec. But the rest is mostly Best Practice. 
#[warn(unused_assignments)]
fn guided_input() -> AppInfo{

    let mut name = String::new(); 
    let mut categories = String::new(); 
    let mut application_type = String::new();
    let mut exec;
    let mut global = String::new();
    let mut icon;
    

    let questions = vec![Question::input("name")
        .message("What is the name of the program?")
        .build(), Question::select("application_type")
        .message("What type of program is it?")
        .choices(vec!["Application", "Link", "Directory"]).default(0)
        .build(), Question::select("categories")
        .message("Which Categorie does it belong to?")
        .choices(vec!["AudioVideo", "Audio", "Video", "Development", "Education", "Game", "Graphics", "Network", "Office", "Settings", "System", "Utility"]).default(11)
        .build(), Question::select("global")
        .message("Do you want to install it globally or only for your current user?")
        .choices(vec!["global", "local"]).default(0)
        .build()];


    let answer = requestty::prompt(questions).unwrap();

    if answer.contains_key("name") {
        name = answer.get("name").unwrap().as_string().unwrap().to_string();

    }

    if answer.contains_key("application_type") {
        application_type = answer.get("application_type").unwrap().as_list_item().unwrap().text.clone();

    }

    if answer.contains_key("categories") {
        categories = answer.get("categories").unwrap().as_list_item().unwrap().text.clone();

    }
    
    if answer.contains_key("global") {
        global = answer.get("global").unwrap().as_list_item().unwrap().text.clone();

    }

    //Check for valid input, if input is invalid ask again instead of quiting.
    loop{
        exec = requestty::prompt_one(
               requestty::Question::input("exec")
                .message("What should be executed")
                .auto_complete(|p, _| auto_complete(p))
                .validate(|p, _| {
                    if (p.as_ref() as &Path).exists() {
                        Ok(())
                    } else {
                        Err(format!("file `{}` doesn't exist", p))
                    }
                }),
            ).unwrap().try_into_string().unwrap();
        if Path::new(&exec.clone()).exists() && exec != ""{
            break;
        }else{
            println!("Error: Path does not exists. Try again!");
        }
    }

    //Check for valid input, if input is invalid ask again instead of quiting.
    loop{
        icon = requestty::prompt_one(
                requestty::Question::input("icon")
                .message("Which icon should be used?")
                .auto_complete(|p, _| auto_complete(p))
                .validate(|p, _| {
                    if (p.as_ref() as &Path).exists() {
                        Ok(())
                    } else {
                        Err(format!("file `{}` doesn't exist", p))
                    }
                }),
            ).unwrap().try_into_string().unwrap();
        
        icon = AppInfo::get_absolute_icon_path(Path::new(&icon));
        if icon != "invalid".to_string() {
            break;
        }
    }

    
    return AppInfo::new(name.clone(), exec, categories.clone(), AppType::convert_app_type(&application_type.clone()).unwrap(), icon, global.clone());

}
