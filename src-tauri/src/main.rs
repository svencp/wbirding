/*
   My attempt to bring the My Free Birding App to bigger screens
   2023-02-16  Sven Ponelat
*/

mod library;

use crate::library::options::SettingsText;
use crate::library::tauri_supp_funcs::*;
use error_feedback::*;

// incoming button bar commands
#[tauri::command]
fn incoming_button_bar(command: &str) -> String {
    return process_button_clicked(command);
}

// initialization of javascript
#[tauri::command]
fn init() -> String {
    return "init baby!".to_string();
}


fn main() {
    // // @@@@@@@ Variables
    // let mut options: SettingsText;

    // // @@@@@@@ Start
    // match SettingsText::bring_in_options() {
    //     Ok(sett) => options = sett,
    //     Err(e) => feedback(Feedback::Error, e),
    // }

    // @@@@@@@ Showing front-end
    println!("This is from the main.rs -> Sven");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![incoming_button_bar, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
