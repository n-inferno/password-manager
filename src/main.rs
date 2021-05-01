mod structures;
mod service;
mod init;
mod new;
mod get;
mod list;
mod change;
mod delete;
mod cryptography;
mod get_config;

use structopt::StructOpt;


fn main() {
    let request = structures::UserCommand::from_args();

    let master_pass = service::get_master_pass();

    if request.command == "init" {
        init::create_new_passwords_file(&master_pass);

    } else if request.command == "new" {
        let resource: String = request.resource.clone().unwrap();
        let psw = service::read_password();
        new::add_password(resource, psw, request.login, request.description, &master_pass);

    } else if request.command == "get" {
        let resource: String = request.resource.clone().unwrap();
        get::print_password(resource, &master_pass);

    } else if request.command == "list" {
        list::print_resources_list(&master_pass);

    } else if request.command == "change" {
        let resource: String = request.resource.clone().unwrap();
        let psw = service::read_password();
        let result = change::change_password(&resource, &psw,
                                             &request.login, &request.description, &master_pass);
        match result {
            true => { println!("Data successfully changed") },
            false => { println!("Resource not found. Try command <new>") }
        }

    } else if request.command == "delete" {
        let resource: String = request.resource.clone().unwrap();
        let result = delete::delete_password(&resource, &master_pass);
        match result {
            true => { println!("Resource successfully deleted") },
            false => { println!("Resource not found") }
        }
    }
}
