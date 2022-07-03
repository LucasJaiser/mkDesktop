
use std::io::stdin;
use std::path::Path;
use std::env;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::converter::convert_categories_number;
use crate::converter::convert_type_number;
use crate::info::AppInfo;
use crate::info::AppType;

mod info;
mod detector;
mod converter;

//Paths for where the actuall .desktop files will go
static GLOBAL_PATH: &str = "/usr/share/applications";
static LOCAL_PATH: &str = "~/.local/share/applications";

#[derive(Parser)]
#[clap(author="Lucas Jaiser", version="1.1", about, long_about = "A CLI tool to create .desktop files with ease")]
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
    AppInfo::write_info_to_file(info, output_path);
    
    confy::store("mkDesktop", cfg).unwrap();

}

///This function gathers information to the .Desktop file in a Guided form. 
///it leads you through all field you will need in the file to be valid. 
///Technically you only need Type, Name and Exec. But the rest is mostly Best Practice. 
fn guided_input() -> AppInfo{

    let mut name = String::new(); 
    let mut categories = String::new(); 
    let mut application_type = String::new();
    let mut exec = String::new();
    let mut global = String::new();
    let mut icon = String::new();

    println!("What name has your programm?");
    stdin().read_line(&mut name).unwrap();
    name = name.trim_end().to_string();

    //Check for valid input, if input is invalid ask again instead of quiting.
    while application_type.ne("Link") && application_type.ne("Application") && application_type.ne("Directory") {
        println!("What type of programm is it? (1 = Application, 2 = Link, 3 = Directory)");
        application_type = "".to_string();
        stdin().read_line(&mut application_type).unwrap();
        application_type = convert_type_number(application_type.trim_end()).unwrap().to_string();
        
    }

    println!("Wich Categorie does it belong to? (Possible values are: 1 = AudioVideo, 2 = Audio, 3 = Video, 4 = Development, 5 = Education, 6 = Game, 7 = Graphics, 8 = Network, 9 = Office, 10 = Settings, 11 = System, 12 = Utility)");
    stdin().read_line(&mut categories).unwrap();
    categories = convert_categories_number(categories.trim_end()).unwrap().to_string();

    //Check for valid input, if input is invalid ask again instead of quiting.
    while !Path::new(&exec.clone()).exists() && exec == "" {
        println!("What should be executed?");
        stdin().read_line(&mut exec).unwrap();
        exec = exec.trim_end().to_string();
    }

    //Check for valid input, if input is invalid ask again instead of quiting.
    while global.ne("global") && global.ne("local") {
        println!("Do you want to install it globally or only for your current user? (valid input: global, local)");
        stdin().read_line(&mut global).unwrap();
        global = global.trim_end().to_string();
    }
    
    //Check for valid input, if input is invalid ask again instead of quiting.
    while icon.eq("") || icon.eq("invalid") {
        println!("Which Icon should be used?");
        stdin().read_line(&mut icon).unwrap();
        icon = icon.trim_end().to_string();
    
        icon = AppInfo::get_absolute_icon_path(Path::new(&icon));
    }
    return AppInfo::new(name.clone(), exec.clone(), categories.clone(), AppType::convert_app_type(&application_type.clone()).unwrap(), icon.clone(), global.clone());

}
