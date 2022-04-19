use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//Paths for where the actuall .desktop files will go
static GLOBAL_PATH: &str = "/usr/share/applications";
static LOCAL_PATH: &str = "~/.local/share/applications";

///Struct wich holds all User input information
#[derive(Clone)]
pub struct AppInfo{
    name: String,
    categories: String, 
    application_type: AppType,
    exec: String,
    global: String,
    icon: String
}

//TODO put in seperate file
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppType{
    Application,
    Link,
    Directory
}


impl AppType{
    //TODO Unittest
    pub fn to_string(self) -> String{
        match self{
            AppType::Application => {return "Application".to_string();},
            AppType::Link => {return "Link".to_string();},
            AppType::Directory => {return "Directory".to_string();},
        }
    }
    
    //TODO Unittest
    pub fn convert_app_type(app_type: &str) -> Result<AppType, String>{
        match app_type{
            "Application" => { return Ok(AppType::Application) },
            "Link" => { return Ok(AppType::Link) },
            "Directory" => { return Ok(AppType::Directory) },
            _ => { return Err("Invalid Application Type".to_string()) }
        }
    }
}

impl AppInfo{
    
    //TODO Unittest
    ///create a new Object of AppInfo
    pub fn new(name: String, exec: String, categories: String, application_type: AppType, icon: String, global: String) -> AppInfo{
        return AppInfo{name: name.clone(), categories: categories.clone(), application_type: application_type.clone(), exec: exec.clone(), global: global.clone(), icon: icon.clone() };
    }
    
    ///Writes the given AppInfo to a actual file
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
        writeln!(file, "Type={}", _info.application_type.to_string().clone()).unwrap();
        writeln!(file, "Icon={}", _info.icon.clone()).unwrap();
    }

    //TODO unittest
    ///Helper function for getting a Absolute path from a Relativ Path
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



