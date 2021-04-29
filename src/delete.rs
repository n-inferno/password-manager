use crate::service;

pub fn delete_password(resource: &String) -> bool {
    let mut json = service::get_all_passwords();
    let index = json.iter().position(|element| element.resource == *resource);
    match index {
        Some(val) => { json.remove(val); service::write_json(json); },
        None => { return false; }
    }
    true
}