use crate::service;

pub fn delete_password(resource: &String, master_pass: &String) -> bool {
    let mut json = service::get_all_passwords(master_pass);
    let index = json.iter().position(|element| element.resource == *resource);
    match index {
        Some(val) => { json.remove(val); service::write_json(json, master_pass); },
        None => { return false; }
    }
    true
}