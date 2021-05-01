use crate::structures::Password;
use std::io::Write;
use crate::cryptography;

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

pub fn get_json(file: String, master_pass: &String) -> Vec<Password> {
    let content_encrypted = std::fs::read(file).unwrap_or_default();
    let mut json: Vec<Password> = Vec::new();
    if !content_encrypted.is_empty() {
    let master_pass_hash = cryptography::get_password_hash(&master_pass);
    let content = cryptography::decrypt(master_pass_hash,
                                        content_encrypted);
        json = serde_json::from_str(&content)
            .expect("JSON was not well-formatted");
    }
    json
}

pub fn write_json(json: Vec<Password>, master_pass: &String) {
    let path = get_file_name(&get_home_dir());
    let text = serde_json::to_string(&json)
        .expect("Invalid json format");
    let master_pass_hash = cryptography::get_password_hash(&master_pass);
    let ciphertext = cryptography::encrypt(master_pass_hash, text);
    std::fs::write(&path, &ciphertext).unwrap();
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

pub fn get_all_passwords(master_pass: &String) -> Vec<Password> {
    let file = get_file_name(&get_home_dir());
    get_json(file, &master_pass)
}

pub fn get_master_pass() -> String {
    print!("Enter your master password: ");
    std::io::stdout().flush().expect("error");
    rpassword::read_password().expect("error reading password")
}