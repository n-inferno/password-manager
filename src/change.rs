use crate::service;

pub fn change_password(resource: &String, new_password: &String,
                       description: &Option<String>) -> bool {
    let mut json = service::get_all_passwords();
    for entry in &mut json {
        if entry.resource == *resource {
            entry.password = new_password.clone();
            match description {
                Some(val) => { entry.description = val.clone() },
                None => ()
            }
            service::write_json(json);
            return true
        }
    }
    false
}
