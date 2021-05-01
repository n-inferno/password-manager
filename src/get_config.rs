use ini::Ini;

pub fn get_secret_str() -> String {
    let file_path = "config.ini";
    let config = Ini::load_from_file(file_path).unwrap();
    config.section(Some("crypto"))
        .unwrap()
        .get("SECRET_KEY")
        .unwrap()
        .to_string()
}