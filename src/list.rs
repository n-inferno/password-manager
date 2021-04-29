use crate::service;

fn get_resources_list() -> Vec<String> {
    let json = service::get_all_passwords();
    let mut result: Vec<String> = Vec::new();
    for entry in json.into_iter() {
        result.push(entry.resource.clone());
    }
    result
}

pub fn print_resources_list() {
    let lst = get_resources_list();
    println!("You've registered following resources:");
    println!("  * {}", lst.join("\n  * "))
}