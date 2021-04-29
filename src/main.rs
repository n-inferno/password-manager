mod structures;
mod service;
mod init;
mod new;
mod get;
mod list;
mod change;
mod delete;

use structopt::StructOpt;


fn main() {
    let request = structures::UserCommand::from_args();

    if request.command == "init" {
        init::create_new_passwords_file();

    } else if request.command == "new" {
        let resource: String = request.resource.clone().unwrap();
        let psw = service::read_password();
        new::add_password(resource, psw, request.description);

    } else if request.command == "get" {
        let resource: String = request.resource.clone().unwrap();
        get::print_password(resource);

    } else if request.command == "list" {
        list::print_resources_list();

    } else if request.command == "change" {
        let resource: String = request.resource.clone().unwrap();
        let psw = service::read_password();
        let result = change::change_password(&resource, &psw,
                                             &request.description);
        match result {
            true => { println!("Data successfully changed") },
            false => { println!("Resource not found. Try command <new>") }
        }

    } else if request.command == "delete" {
        let resource: String = request.resource.clone().unwrap();
        let result = delete::delete_password(&resource);
        match result {
            true => { println!("Resource successfully deleted") },
            false => { println!("Resource not found") }
        }
    }
}
