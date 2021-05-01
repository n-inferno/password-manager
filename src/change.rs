use crate::service;

pub fn change_password(resource: &String, new_password: &String, login: &Option<String>,
                       description: &Option<String>, master_pass: &String) -> bool {
    let mut json = service::get_all_passwords(master_pass);
    for entry in &mut json {
        if entry.resource == *resource {
            entry.password = new_password.clone();
            match description {
                Some(val) => { entry.description = val.clone() },
                None => ()
            }
            match login {
                Some(val) => { entry.login = val.clone() },
                None => ()
            }
            service::write_json(json, master_pass);
            return true
        }
    }
    false
}
