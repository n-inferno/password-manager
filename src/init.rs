use std::fs;
use crate::service;

pub fn create_new_passwords_file() {
    let home_dir = service::get_home_dir();
    service::create_dir(&home_dir);
    let file = fs::File::create(service::get_file_name(&home_dir));
    match file {
        Ok(res) => println!("Successfully created {:?}", res),
        Err(e) => println!("Failed to create file, {}", e)
    }
}