/*
   My attempt to bring the My Free Birding App to bigger screens
   2023-02-16  Sven Ponelat
*/

mod library;

use crate::library::options::SettingsText;
use error_feedback::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // @@@@@@@ Variables
    let mut options: SettingsText;

    // @@@@@@@ Start
    match SettingsText::bring_in_options() {
        Ok(sett) => options = sett,
        Err(e) => feedback(Feedback::Error, e),
    }

    println!("This is from the main.rs -> Sven");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
