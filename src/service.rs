use crate::structures::Password;
use std::fs::File;
use std::io::Write;


pub fn get_file_name(home_dir: &String) -> String {
    let username = whoami::username();
    format!("{}{}.json", home_dir, username)
}

pub fn create_dir(home_dir: &String) {
    match std::fs::create_dir(&home_dir) {
        _ => ()
    };
}

pub fn get_home_dir() -> String {
    let mut home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();
    home_dir.push_str("/.password_manager/");
    home_dir
}

pub fn get_json(file: String) -> Vec<Password> {
    let content = std::fs::read_to_string(file).unwrap_or_default();
    let mut json: Vec<Password> = Vec::new();
    if !content.is_empty() {
        json = serde_json::from_str(&content)
            .expect("JSON was not well-formatted");
    }
    json
}

pub fn write_json(json: Vec<Password>) {
    let path = get_file_name(&get_home_dir());
    let descriptor = File::create(path).unwrap();
    serde_json::to_writer_pretty(descriptor, &json)
        .expect("Invalid json format");
}


pub fn read_password() -> String {
    print!("Enter the password: ");
    std::io::stdout().flush().expect("error");
    loop {
        let psw1: String = rpassword::read_password().unwrap();
        print!("Enter the password again: ");
        std::io::stdout().flush().expect("error");
        let psw2: String = rpassword::read_password().unwrap();
        if psw1 == psw2 {
            break psw1;
        }
        print!("Passwords are different. Please try again: ");
        std::io::stdout().flush().expect("error");
    }
}

pub fn get_all_passwords() -> Vec<Password> {
    let file = get_file_name(&get_home_dir());
    get_json(file)
}