use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///Struct wich holds all User input information
#[derive(Clone)]
pub struct AppInfo{
    name: String,
    categories: String, 
    application_type: AppType,
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
    
    ///Writes the given AppInfo to a actual file
    pub fn write_info_to_file(_info: AppInfo, path: String){

        //Create the file and write all Informations we have to it.
        let mut file = File::create(path + &"/".to_string() + &_info.name.clone() + ".desktop").unwrap();

        writeln!(file, "{}", "[Desktop Entry]").unwrap();
        writeln!(file, "{}", "Version=1.0").unwrap();
        writeln!(file, "Name={}", _info.name.clone()).unwrap();
        writeln!(file, "Exec={}", _info.exec.clone()).unwrap();
        writeln!(file, "Categories={}", _info.categories.clone()).unwrap();
        writeln!(file, "Type={}", _info.application_type.to_string().clone()).unwrap();
        writeln!(file, "Icon={}", _info.icon.clone()).unwrap();
    }

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
        assert_eq!(AppType::convert_app_type("Application").unwrap(), AppType::Application);
        assert_eq!(AppType::convert_app_type("Link").unwrap(), AppType::Link);
        assert_eq!(AppType::convert_app_type("Directory").unwrap(), AppType::Directory);
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


