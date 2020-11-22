use crate::account_parser::{get_account_file, parse_account_file};
pub fn print_config(){
    if !std::path::Path::new(&get_account_file()).exists(){
        println!("💁 Seems your not logged in! Register an account with 'starshipper register'!");
        std::process::exit(1);
    }
    let creds = parse_account_file();
    println!("👤 Username: {}", creds["username"].to_string().replace("\"", ""));
    println!("🔑 Password: {}", creds["password"].to_string().replace("\"", ""));
}
