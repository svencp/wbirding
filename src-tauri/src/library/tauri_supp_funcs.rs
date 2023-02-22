/*
   my support file for all tauri commands
   2023-02-22  Sven Ponelat
*/

use crate::library::options::*;


pub fn process_button_clicked(command: &str) -> String {
    match command {
        "button-bar-save" => return "Save button was clicked!".to_string(),
        "button-bar-increase" => {
            let result = increase_fontSize_in_files();
            let mut options = SettingsText::new();
            match SettingsText::bring_in_options() {
                Ok(sett) => {
                    options = sett;
                }
                Err(_) => return "Error in opening Options.txt file.".to_string(),
            }
            let result = options.get_integer("fontSize");
            let num = result.unwrap() + 1;

            return "button-bar-increase".to_string();
        }
        "button-bar-decrease" => return "button-bar-decrease".to_string(),
        "button-bar-clear" => return "clear button was clicked!".to_string(),
        "button-bar-exit" => return "exit button was clicked!".to_string(),
        _ => return "Unrecognized button clicked".to_string(),
    }

    // return "".to_string();
}


//@@@@@@@ Support functions
//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
pub fn increase_fontSize_in_files() -> Result<(),String> {
    
    
    
    return Err("No way".to_string())
}







