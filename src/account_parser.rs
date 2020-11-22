use toml::Value;
use std::fs::File;
use std::io::Read;
pub fn parse_account_file() -> Value{
    let mut data_file = File::open(get_account_file()).unwrap();
    let mut data = String::from("");
    match data_file.read_to_string(&mut data){
        Ok(_) => print!(""),
        Err(err) => println!("Failed to read {}! Error:\n{}", get_account_file(), err)
    }
    let toml = data.parse::<Value>().unwrap();
    return toml;
}
pub fn get_account_file() -> String {
    return format!("{}/.starshipper-creds.toml", std::env::var("HOME").unwrap())
}
