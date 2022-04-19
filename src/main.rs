use std::io::stdin;
use std::path::Path;

use clap::Parser;

use crate::info::AppInfo;
use crate::info::AppType;

mod info;


#[derive(Parser)]
#[clap(author="Lucas Jaiser", version="1.0", about, long_about = "A CLI tool to create .desktop files with ease")]
struct Cli {
    /// Name of the File you want to create
    #[clap(short, long)]
    name: String,

    ///Application Type (possible values: Application, Link, Directory)
    #[clap(short, long, parse(try_from_str=AppType::convert_app_type))]
    app_type: AppType,

    ///Categories wich describes your Application, you can find possible Categories here: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
    #[clap(short, long, default_value_t = String::from(""))]
    categories: String,

    ///The binary or .sh etc. which should be executed
    #[clap(short, long)]
    exec: String,
    
    ///The Icon wich will be displayed with this Application
    #[clap(short, long)]
    icon: String,
    
    ///Should mkDesktop install the in global Directory or in the Local only for the current user
    #[clap(short, long, default_value_t = String::from("global"))]
    global: String,

    ///Starts the guided mode of mkDesktop, you will get asked step by step all needed Information.
    ///Good for beginners 
    #[clap(short = 'G', long)]
    guided: bool,
}


fn main() {
    let cli = Cli::parse();

    if cli.guided {
        println!("---------------Guided Mode----------------");
        //Start guided Input mode, this is where the information to the .Desktop file is gathered.
        let info: AppInfo = guided_input();

        //takes the struct and writes it to the actual file in the correct Location based on input 
        AppInfo::write_info_to_file(info)
    }
   

    let info: AppInfo = AppInfo::new(cli.name, cli.exec, cli.categories, cli.app_type, cli.icon, cli.global);
    
    AppInfo::write_info_to_file(info);

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
        println!("What type of programm is it? (Application, Link, Directory)");
        application_type = "".to_string();
        stdin().read_line(&mut application_type).unwrap();
        application_type = application_type.trim_end().to_string();
    }

    println!("Wich Categorie does it belong to? (Possible values are: AudioVideo, Audio, Video, Development, Education, Game, Graphics, Network, Office, Settings, System, Utility)");
    stdin().read_line(&mut categories).unwrap();
    categories = categories.trim_end().to_string();

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
