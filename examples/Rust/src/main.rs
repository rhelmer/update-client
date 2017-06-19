#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct ResultMessage {
    update_type: String,
    download_path: String,
}

fn main() {
    println!("\n-----------------------------------------------------\n");
    println!("This is an example app to exercise the update client!\n");
    println!("Checking for updates...\n");

    let output = Command::new("../../target/release/update_client")
        .output()
        .expect("failed to execute process");

    let json_str = String::from_utf8_lossy(&output.stdout);
    let result_message: ResultMessage = serde_json::from_str(&json_str).unwrap();

    println!("{} update is ready to apply at {}\n",
        result_message.update_type,
        result_message.download_path);

    println!("Applying update for {}...", result_message.update_type);

    fs::copy(result_message.download_path,
        format!("./updates/{}", result_message.update_type)).unwrap();

    println!("{} update has been applied in ./updates/{}\n",
        result_message.update_type,
        result_message.update_type);

    println!("All done!");
    println!("\n-----------------------------------------------------\n");

}
