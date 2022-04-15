use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static GLOBAL_PATH: &str = "/usr/share/applications";
static LOCAL_PATH: &str = "~/.local/share/applications";

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
    
    pub fn new(name: String, exec: String, categories: String, application_type: String, icon: String, global: String) -> AppInfo{
        return AppInfo{name: name.clone(), categories: categories.clone(), application_type: application_type.clone(), exec: exec.clone(), global: global.clone(), icon: icon.clone() };
    }

    pub fn write_info_to_file(_info: AppInfo){

        let file_path: &str;
        if _info.global == "global" {
            file_path = GLOBAL_PATH;
        }else {
            file_path = LOCAL_PATH;
        }

        let mut file = File::create(file_path.to_string() + &"/".to_string() + &_info.name.clone() + ".desktop").unwrap();
        let content:String = "[Desktop Entry]".to_string();
        writeln!(file, "{}", content.clone()).unwrap();
        writeln!(file, "{}", "Version=1.0").unwrap();
        writeln!(file, "Name={}", _info.name.clone()).unwrap();
        writeln!(file, "Exec={}", _info.exec.clone()).unwrap();
        writeln!(file, "Categories={}", _info.categories.clone()).unwrap();
        writeln!(file, "Type={}", _info.application_type.clone()).unwrap();
        writeln!(file, "Icon={}", _info.icon.clone()).unwrap();
    }

    pub fn get_absolute_icon_path(icon_path: &Path) -> String{

        if !icon_path.exists() {
            println!("Path to file {} does not exist!", icon_path.to_str().unwrap());
            panic!();
        }else {
            //Get absolute icon path
            return icon_path.canonicalize().unwrap().to_str().unwrap().to_string();
        }

    }
}



