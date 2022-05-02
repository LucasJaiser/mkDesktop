use std::fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::{PathBuf};

use crate::info::{AppInfo, AppType};

/// This function detects every information it can get from the current directory which is defined by path parameter. 
/// AppInfo::name will be  the directory name
/// AppInfo::icon will be the file with a png or jpeg extension
/// AppInfo::exec will be the file with executable permissions
/// Returns a AppInfo struct with detected information 
pub fn detect(path: PathBuf) -> Result<AppInfo, String>{
    let paths = fs::read_dir(path.clone()).unwrap();
    let path_name: String = String::from(path.as_path().file_name().unwrap().to_str().unwrap());
    let mut icon: String = String::from("");
    let mut exec: String = String::from("");
    
    for elem in paths {

        let dir = elem.unwrap();
        
        let buf = dir.path();
        if buf.is_file() {
           
            let ext = buf.extension();

            match ext {
                None => (),
                Some(ext) => {
                    if ext.to_str().unwrap() == "png" {
                        icon = buf.display().to_string();
                    }else if ext.to_str().unwrap() == "jpeg" {
                        icon = buf.display().to_string();
                    }
                },
            }

            let mode = dir.metadata().unwrap().permissions().mode();
            if mode & 0o111 != 0 {
                exec = String::from(buf.file_name().unwrap().to_str().unwrap());
            }
            
        }
    }

    if path_name == String::from(""){
        return Err(String::from("Could not find Application Name!"));
    }
    if icon == String::from(""){
        return Err(String::from("Could not find icon!"));
    }
    if exec == String::from(""){
        return Err(String::from("could not find executable!"));
    }

    return Ok(AppInfo::new(path_name, exec, String::from(""), AppType::Application, icon, String::from("global")));
    
}
