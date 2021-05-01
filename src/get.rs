use crate::structures::Password;
use crate::service;

pub fn get_password(resource: &String, master_pass: &String) -> Option<Password> {
    let json = service::get_all_passwords(master_pass);
    let mut result: Option<Password> = None;
    for entry in json.into_iter() {
        if &entry.resource == resource {
            result = Some(entry); break;
        }
    }
    result
}

pub fn print_password(resource: String, master_pass: &String) {
    let result = get_password(&resource, master_pass);
    let password = match result {
        None => { println!("Resource not found"); return; },
        Some(val) => val
    };
    println!("  * Resource: {}", &password.resource);
    if !password.login.is_empty() {
        println!("  * Login: {}", &password.login);
    }
    println!("  * Password: {}", &password.password);
    if !password.description.is_empty() {
        println!("  * Description: {}", &password.description);
    }

}