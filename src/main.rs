use std::io::stdin;
use std::path::Path;

use crate::info::AppInfo;

mod info;


fn main() {
    //Start guided Input mode, this is where the information to the .Desktop file is gathered.
    let info: AppInfo = guided_input();
    //takes the struct and writes it to the actual file in the correct Location based on input 
    AppInfo::write_info_to_file(info)
}

//this function gathers information to the .Desktop file in a Guided form. 
//it leads you through all field you will need in the file to be valid. 
//Technically you only need Type, Name and Exec. But the rest is mostly Best Practice. 
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

    println!("What type of programm is it? (Application, Link, Directory)");
    stdin().read_line(&mut application_type).unwrap();
    application_type = application_type.trim_end().to_string();

    println!("Wich Categorie does it belong to? (Possible values are: AudioVideo, Audio, Video, Development, Education, Game, Graphics, Network, Office, Settings, System, Utility)");
    stdin().read_line(&mut categories).unwrap();
    categories = categories.trim_end().to_string();

    println!("What should be executed?");
    stdin().read_line(&mut exec).unwrap();
    exec = exec.trim_end().to_string();

    println!("Do you want to install it globally or only for your current user?");
    stdin().read_line(&mut global).unwrap();
    global = global.trim_end().to_string();

    println!("Which Icon should be used?");
    stdin().read_line(&mut icon).unwrap();
    icon = icon.trim_end().to_string();
    
    let _icon_path = AppInfo::get_absolute_icon_path(Path::new(&icon));
    return AppInfo::new(name.clone(), exec.clone(), categories.clone(), application_type.clone(), _icon_path.clone(), global.clone());

}
