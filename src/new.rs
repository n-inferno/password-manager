use crate::service;
use crate::structures::Password;

pub fn add_password(resource: String, password: String, login: Option<String>,
                    description: Option<String>, master_pass: &String) {
    let file = service::get_file_name(&service::get_home_dir());
    if !std::path::Path::new(&file).exists() {
        println!("You didn't init the password file. Try 'init' command");
        return
    }
    let mut curr_content = service::get_json(file, master_pass);
    let description = match description {
        Some(desc) => desc,
        None => String::new()
    };
    let login = match login {
        Some(log) => log,
        None => String::new()
    };

    let new_entry = Password { resource, password, login, description };

    if check_if_pass_exists(&curr_content, new_entry.clone()) {
        println!("Resource already exists. Try to use command <change>");
        return
    }

    curr_content.push(new_entry);
    service::write_json(curr_content, master_pass);
    println!("Password successfully saved");
}

pub fn check_if_pass_exists(curr_content: &Vec<Password>,
                        new_entry: Password) -> bool {
    let mut result = false;
    for entry in curr_content.into_iter() {
        if entry.resource == new_entry.resource {
            result = true
        }
    }
    result
}