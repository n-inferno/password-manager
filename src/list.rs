use crate::service;

fn get_resources_list(master_pass: &String) -> Vec<String> {
    let json = service::get_all_passwords(master_pass);
    let mut result: Vec<String> = Vec::new();
    for entry in json.into_iter() {
        result.push(entry.resource.clone());
    }
    result
}

pub fn print_resources_list(master_pass: &String) {
    let lst = get_resources_list(master_pass);
    if lst.is_empty() {
        println!("No resources found");
        return;
    }
    println!("You've registered following resources:");
    println!("  * {}", lst.join("\n  * "))
}