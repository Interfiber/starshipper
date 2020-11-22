use text_io::*;
use indicatif::{ ProgressBar, ProgressStyle};
use std::fs::write;
pub async fn retrive_file() -> Result<(), reqwest::Error>{
    println!("💁 To download a file you will need to UUID that the uploader gave you");
    println!("🔑 Enter UUID:");
    let uuid: String = read!();
    println!("📄 Enter the path to save the file:");
    let save_file_path: String = read!();
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(500);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("🐵🙊🙈🙉")
            .template("{spinner:.dim.bold} {wide_msg}"),
    );
    pb.set_message("Fetching file...");
    let client = reqwest::blocking::Client::new();
    let body = client.get(&format!("http://{}:5656/download", std::env::var("LOCAL_COMPUTER").unwrap())).header("uuid", uuid.to_string()).send()?;
    let text = body.text().unwrap();
    pb.set_message("Writing file...");
    write(save_file_path.to_string(), text).expect("Failed to write to file");
    pb.set_message("Deleting file from server...");
    let _ = client.get(&format!("http://{}:5656/delete", std::env::var("LOCAL_COMPUTER").unwrap())).header("uuid", uuid.to_string()).send()?;
    pb.finish_and_clear();
    println!("✨ File Transfered to {}!", save_file_path.to_string());
    Ok(())
}
