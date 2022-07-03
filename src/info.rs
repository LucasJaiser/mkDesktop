use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::process::Command;

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
        result.push_str("echo \"[Desktop Entry]\n");
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
        result.push_str("\"");
        return result;
    }
    
    ///Writes the given AppInfo to a actual file
    pub fn write_info_to_file(_info: AppInfo, path: String){
       
        let info_string = AppInfo::convert_to_string(_info.clone()); 
        //Create the file and write all Informations we have to it
        if _info.global == "global" {
            //We are writing to /usr/share so we need sudo rights to create and write to a file 
            Command::new("sudo").args(["touch", (path.clone() + "/" + &_info.name.clone() + ".desktop").as_str()]).output().unwrap();
            Command::new("sh").args(["-c", (info_string.clone() + " | sudo tee " + &path.clone() + "/" + &_info.name.clone() + ".desktop").as_str()]).output().unwrap();
        }else{
            let mut file = OpenOptions::new().write(true).create(true).open(path + "/" + &_info.name.clone() + ".desktop").unwrap();

            writeln!(file, "{}", info_string).unwrap();
        }
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
    use crate::converter::convert_categories_number;
    use crate::converter::convert_type_number;

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

    #[test]
    fn test_convert_number_type() {
        assert_eq!(String::from("Application"), convert_type_number("1").unwrap());
        assert_eq!(String::from("Link"), convert_type_number("2").unwrap());
        assert_eq!(String::from("Directory"), convert_type_number("3").unwrap());
        assert_eq!(String::from("TestTest"), convert_type_number("TestTest").unwrap());

    }

    #[test]
    fn test_convert_number_categorie() {
        assert_eq!(String::from("AudioVideo"), convert_categories_number("1").unwrap());
        assert_eq!(String::from("Audio"), convert_categories_number("2").unwrap());
        assert_eq!(String::from("Video"), convert_categories_number("3").unwrap());
        assert_eq!(String::from("Development"), convert_categories_number("4").unwrap());
        assert_eq!(String::from("Education"), convert_categories_number("5").unwrap());
        assert_eq!(String::from("Game"), convert_categories_number("6").unwrap());
        assert_eq!(String::from("Graphics"), convert_categories_number("7").unwrap());
        assert_eq!(String::from("Network"), convert_categories_number("8").unwrap());
        assert_eq!(String::from("Office"), convert_categories_number("9").unwrap());
        assert_eq!(String::from("Settings"), convert_categories_number("10").unwrap());
        assert_eq!(String::from("System"), convert_categories_number("11").unwrap());
        assert_eq!(String::from("Utility"), convert_categories_number("12").unwrap());
        assert_eq!(String::from("TestTest"), convert_categories_number("TestTest").unwrap());
    }
}


