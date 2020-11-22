use text_io::*;
use curl::easy::{Easy, List};
use std::fs::write;
pub fn login(username: String, password: String){
    write(&format!("{}/.starshipper-creds.toml", std::env::var("HOME").unwrap()), &format!("username = \"{}\"\npassword = \"{}\"", username, password)).unwrap();
}
pub fn new_account(){
        println!("👋 Welcome to the Starshipper new account wizard!");
        println!("💁 Usernames and passwords do not support spaces or emojis");
        println!("👤 Enter your account name:");
        let username: String = read!();
        println!("🔑 Enter your password:");
        let password: String = read!();
        println!("💁 An account named {} will be created with the password {}. Do you wish to create it(y/n)?", username, password);
        let proceed: String = read!();
        if proceed == "y" {
            println!("📲 Creating account...");
            let mut easy = Easy::new();
            easy.url(&format!("{}:5656/create_account", std::env::var("LOCAL_COMPUTER").unwrap())).unwrap();
            let mut list = List::new();
            list.append(&format!("password: {}", password)).unwrap();
            list.append(&format!("username: {}", username)).unwrap();
            easy.http_headers(list).unwrap();
            easy.perform().unwrap();
            login(username, password);
            println!("✨ Account Created!");
        }
}
