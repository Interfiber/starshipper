use text_io::*;
use indicatif::{ ProgressBar, ProgressStyle};
use std::fs::File;
use uuid::Uuid;
use curl::easy::{Easy, List};
use std::io::{self, prelude::*, BufReader};
pub fn send_file() -> io::Result<()>{
    println!("💁 Make sure you are logged into starshipper first. Or the file transfer will not complete!");
    println!("📄 Enter file to transfer:");
    let file: String = read!();
    println!("💁 Are you sure you want to send the file(y/n)?");
    let send: String = read!();
    if send == "y" {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(500);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("🐵🙊🙈🙉")
                .template("{spinner:.dim.bold} {wide_msg}"),
        );
        pb.set_message("Generating UUID...");
        let uuid = Uuid::new_v4();
        pb.set_message(&format!("Uploading {}...", file.to_string()));
        let file2 = File::open(file.to_string())?;
        let reader = BufReader::new(file2);
        let account = crate::account_parser::parse_account_file();
        for line in reader.lines() {
            let mut easy = Easy::new();
            easy.url(&format!("{}:5656/save", std::env::var("LOCAL_COMPUTER").unwrap())).unwrap();
            let mut list = List::new();
            list.append(&format!("filename: {}", uuid)).unwrap();
            list.append(&format!("line: {}", line?)).unwrap();
            easy.http_headers(list).unwrap();
            easy.perform().unwrap();
        }

        pb.finish_and_clear();

        println!("💁 The file was uploaded! To get it you can run 'starshipper retrive'. After the file is downloaded it will be deleted.");
        println!("🔑 The ID is {}", uuid);

    }
    Ok(())
}
