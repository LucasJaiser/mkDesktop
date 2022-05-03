


pub fn convert_categories_number(number: &str) -> Result<String, String>{

    match number{
        "1" => { return Ok(String::from("AudioVideo")) },
        "2" => { return Ok(String::from("Audio")) },
        "3" => { return Ok(String::from("Video")) },
        "4" => { return Ok(String::from("Development")) },
        "5" => { return Ok(String::from("Education")) },
        "6" => { return Ok(String::from("Game")) },
        "7" => { return Ok(String::from("Graphics")) },
        "8" => { return Ok(String::from("Network")) },
        "9" => { return Ok(String::from("Office")) },
        "10" => { return Ok(String::from("Settings")) },
        "11" => { return Ok(String::from("System")) },
        "12" => { return Ok(String::from("Utility")) },
        _ => { return Ok(number.to_owned())}
    }

}

pub fn convert_type_number(number: &str) -> Result<String, String>{
    match number{
        "1" => { return Ok(String::from("Application")) },
        "2" => { return Ok(String::from("Link")) },
        "3" => { return Ok(String::from("Directory")) },
        _ => { return Ok(number.to_owned())}
    }
}