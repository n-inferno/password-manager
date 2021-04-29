mod structures;
mod service;
mod init;
mod new;
use structopt::StructOpt;


fn main() {
    let request = structures::UserCommand::from_args();
    println!("{:#?}", &request);

    if request.command == "init" {
        init::create_new_passwords_file();

    } else if request.command == "new" {
        let resource: String = request.resource.clone().unwrap();
        let psw = service::read_password();
        new::add_password(resource, psw, request.description);

    } else if request.command == "get" {
        unimplemented!()
    } else if request.command == "list" {
        unimplemented!()
    } else if request.command == "change" {
        unimplemented!()
    } else if request.command == "delete" {
        unimplemented!()
    }
}


// commands:
// init - инициализирует файл в папке ~/.password_manager
// new -d <description> resource - создать новый пароль
// get resource - получить пароль ресурса;
//     -d также получить описание ресурса
// list - список все ресурсов
// change resource - изменить пароль ресурса;
//     -d <description> также изменить описание ресурса
// delete resource - удалить
