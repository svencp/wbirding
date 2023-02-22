/*
   My attempt to bring the My Free Birding App to bigger screens
   2023-02-16  Sven Ponelat
*/

mod library;

use crate::library::options::SettingsText;
use error_feedback::*;

// increase the base font
#[tauri::command]
fn incoming_command(command: &str) -> String {
    match command {
        "button-bar-save" => { 
            return "Save button was clicked!".to_string() 
        }
        "button-bar-increase" => { 
            let mut options = SettingsText::new();
            match SettingsText::bring_in_options() {
                Ok(sett) => options = sett,
                Err(e) => feedback(Feedback::Error, e),
            }
            let result = options.get_integer("fontSize");
            let num = result.unwrap() + 1;
            
            return "button-bar-increase".to_string() 
        }
        "button-bar-decrease" => { 
            return "decrease button was clicked!".to_string() 
        }
        "button-bar-clear" => { 
            return "clear button was clicked!".to_string() 
        }
        "button-bar-exit" => { 
            return "exit button was clicked!".to_string() 
        }
        _ => {
            return "Unrecognized button clicked".to_string()
        }
        
    }

    // let str = command;
    // // match len {
    // //     1..=5 => {
    // //         return "All Good".to_string()
    // //     }
    // //     _ => {
    // //         return "Not soo good".to_string()
    // //     }
    // // }

    // format!("You go {}",str)
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
        .invoke_handler(tauri::generate_handler![incoming_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
