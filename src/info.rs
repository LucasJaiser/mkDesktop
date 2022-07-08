use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::process::Command;

use spinach::Spinach;

///Struct wich holds all User input information
#[derive(Clone)]
pub struct AppInfo{
    name: String,
    pub categories: String, 
    pub application_type: AppType,
    exec: String,
    pub global: String,
    icon: String
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AppType{
    Application,
    Link, 
    Directory
}

impl AppType{
    ///Converts a AppType enum to String
    pub fn to_string(self) -> String{
        match self{
            AppType::Application => {return "Application".to_string();},
            AppType::Link => {return "Link".to_string();},
            AppType::Directory => {return "Directory".to_string();},
        }
    }
    
    ///Converts a String to a AppType enum
    pub fn convert_app_type(app_type: &String) -> Result<AppType, String>{
        match app_type.as_str(){
            "Application" => { return Ok(AppType::Application) },
            "Link" => { return Ok(AppType::Link) },
            "Directory" => { return Ok(AppType::Directory) },
            _ => { return Err("Invalid Application Type".to_string()) }
        }
    }
}

impl AppInfo{
    

    ///create a new Object of AppInfo
    pub fn new(name: String, exec: String, categories: String, application_type: AppType, icon: String, global: String) -> AppInfo{
        return AppInfo{name: name.clone(), categories: categories.clone(), application_type: application_type.clone(), exec: exec.clone(), global: global.clone(), icon: icon.clone() };
    }
    
    pub fn convert_to_string(info: AppInfo) -> String{
        let mut result: String = "".to_string();
        result.push_str("[Desktop Entry]\n");
        result.push_str("Version=1.0");
        result.push_str("\n");
        result.push_str("Name=");
        result = result + &info.name.clone();
        result.push_str("\n");
        result.push_str("Exec=");
        result = result + &info.exec.clone();
        result.push_str("\n");
        result.push_str("Categories=");
        result = result + &info.categories.clone();
        result.push_str("\n");
        result.push_str("Type=");
        result = result + &info.application_type.to_string().clone();
        result.push_str("\n");
        result.push_str("Icon=");
        result = result + &info.icon.clone();
        return result;
    }
    
    ///Writes the given AppInfo to a actual file
    pub fn write_info_to_file(_info: AppInfo, path: String, output: bool){
       
        let mut info_string = AppInfo::convert_to_string(_info.clone()); 

        //Only out put the info_string since --output flag is set
        if output {
            println!("{}", info_string);
            return;
        }
        
        //prepare info_string for echo output
        info_string = "echo \"".to_string() + &info_string;
        info_string.push_str("\"");
        //Create the file and write all Informations we have to it
        if _info.global == "global" {
            //We are writing to /usr/share so we need sudo rights to create and write to a file 
            Command::new("sudo").args(["touch", (path.clone() + "/" + &_info.name.clone() + ".desktop").as_str()]).output().unwrap();
             let s = Spinach::new("Creating .desktop file...");

            Command::new("sh").args(["-c", (info_string.clone() + " | sudo tee " + &path.clone() + "/" + &_info.name.clone() + ".desktop").as_str()]).output().unwrap();
            s.succeed("Succesfully created file!");

        }else{
            let s = Spinach::new("Creating .desktop file...");

            let mut file = OpenOptions::new().write(true).create(true).open(path + "/" + &_info.name.clone() + ".desktop").unwrap();

            writeln!(file, "{}", info_string).unwrap();
            s.succeed("Succesfully created file!");

        }
    }

    ///Helper function for getting a Absolute path from a Relativ Path
    pub fn get_absolute_icon_path(icon_path: &Path) -> String{
        if !icon_path.exists() { //First check if the file even is existing.
            println!("Error: Path to file {} does not exist!", icon_path.to_str().unwrap());
            return "invalid".to_string();
        }else {
            //Convert to absolute path
            return icon_path.canonicalize().unwrap().to_str().unwrap().to_string();
        }

    }

    pub fn print_template(){
        
        //Create the file and write all Informations we have to it.
        let mut file = File::create("template.desktop").unwrap();

        writeln!(file, "{}", "[Desktop Entry]").unwrap();
        writeln!(file, "{}", "Version=1.0").unwrap();
        writeln!(file, "Name={}", "").unwrap();
        writeln!(file, "Exec={}", "").unwrap();
        writeln!(file, "Categories={}", "").unwrap();
        writeln!(file, "Type={}", "").unwrap();
        writeln!(file, "Icon={}", "").unwrap();
    }

}

#[cfg(test)]
mod test{
    
    use crate::AppType; 
    use crate::AppInfo;
          

    #[test]
    fn test_app_type_to_string(){
        

        let app: AppType = AppType::Application;
        assert_eq!(AppType::to_string(app),  String::from("Application"));
        assert_ne!(AppType::Link.to_string(),  String::from("Application"));
    }

    #[test]
    fn test_app_type_convert_to_enum(){
        assert_eq!(AppType::convert_app_type(&String::from("Application")).unwrap(), AppType::Application);
        assert_eq!(AppType::convert_app_type(&String::from("Link")).unwrap(), AppType::Link);
        assert_eq!(AppType::convert_app_type(&String::from("Directory")).unwrap(), AppType::Directory);
    }

    #[test]
    fn test_app_info_new(){
        let info: AppInfo = AppInfo::new(String::from("name"), String::from("exec"), String::from("categories"), AppType::Application, String::from("icon"), String::from("global"));
        assert_eq!(String::from("name"), info.name);
        assert_eq!(String::from("exec"), info.exec);
        assert_eq!(String::from("categories"), info.categories);
        assert_eq!(AppType::Application, info.application_type);
        assert_eq!(String::from("icon"), info.icon);
        assert_eq!(String::from("global"), info.global);
    }

   }


