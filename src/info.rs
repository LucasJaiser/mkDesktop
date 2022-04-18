use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//Paths for where the actuall .desktop files will go
static GLOBAL_PATH: &str = "/usr/share/applications";
static LOCAL_PATH: &str = "~/.local/share/applications";

//Struct wich holds all User input information
#[derive(Clone)]
pub struct AppInfo{
    name: String,
    categories: String, 
    application_type: String,
    exec: String,
    global: String,
    icon: String
}

impl AppInfo{
    
    //create a new Object of AppInfo
    pub fn new(name: String, exec: String, categories: String, application_type: String, icon: String, global: String) -> AppInfo{
        return AppInfo{name: name.clone(), categories: categories.clone(), application_type: application_type.clone(), exec: exec.clone(), global: global.clone(), icon: icon.clone() };
    }
    
    //Writes the given AppInfo to a actual file
    pub fn write_info_to_file(_info: AppInfo){

        //figure out in wich folder we want to create the file
        let file_path: &str;
        if _info.global == "global" {
            file_path = GLOBAL_PATH;
        }else {
            file_path = LOCAL_PATH;
        }

        //Create the file and write all Informations we have to it.
        let mut file = File::create(file_path.to_string() + &"/".to_string() + &_info.name.clone() + ".desktop").unwrap();

        writeln!(file, "{}", "[Desktop Entry]").unwrap();
        writeln!(file, "{}", "Version=1.0").unwrap();
        writeln!(file, "Name={}", _info.name.clone()).unwrap();
        writeln!(file, "Exec={}", _info.exec.clone()).unwrap();
        writeln!(file, "Categories={}", _info.categories.clone()).unwrap();
        writeln!(file, "Type={}", _info.application_type.clone()).unwrap();
        writeln!(file, "Icon={}", _info.icon.clone()).unwrap();
    }

    //Helper function for getting a Absolute path from a Relativ Path
    pub fn get_absolute_icon_path(icon_path: &Path) -> String{


        if !icon_path.exists() { //First check if the file even is existing.
            println!("Path to file {} does not exist!", icon_path.to_str().unwrap());
            return "invalid".to_string();
        }else {
            //Convert to absolute path
            return icon_path.canonicalize().unwrap().to_str().unwrap().to_string();
        }

    }
}



