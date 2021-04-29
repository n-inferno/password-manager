use std::fs;
use crate::service;

pub fn create_new_passwords_file() {
    let home_dir = service::get_home_dir();
    service::create_dir(&home_dir);
    let file_name = service::get_file_name(&home_dir);
    match std::path::Path::new(&file_name).exists() {
        true => { println!("File already exists"); return; },
        false => ()
    };
    fs::File::create(file_name).expect("Failed to create file");
}