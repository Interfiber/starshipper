use clap::{App, SubCommand, AppSettings};
// Declare mods
mod login;
mod config;
mod send;
mod account_parser;
mod retrive;
#[tokio::main]

async fn main() {
    let app = App::new("🌌 🚀 Starshipper")
                       .version("0.1")
                       .author("Interfiber <https://interfiber.github.io>")
                       .about("Transfer files over your local network")
                       .setting(AppSettings::ArgRequiredElseHelp)
                       .subcommand(SubCommand::with_name("register")
                                   .about("🔓 Register for starshipper account"))
                        .subcommand(SubCommand::with_name("config")
                                    .about("🕵️  Print config for your starshipper account"))
                        .subcommand(SubCommand::with_name("send")
                                    .about("⬆️  Send someone a file"))
                        .subcommand(SubCommand::with_name("retrive")
                                    .about("🚚 Retrive a file someone uploaded"))
                       .get_matches();
if let Some(_app) = app.subcommand_matches("register") {
    login::new_account();
 }
 if let Some(_app) = app.subcommand_matches("config") {
     config::print_config();
  }
  if let Some(_app) = app.subcommand_matches("send") {
      send::send_file().unwrap();
   }
   if let Some(_app) = app.subcommand_matches("retrive") {
       let _ = retrive::retrive_file().await;
   }
}
