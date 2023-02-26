/*
   My attempt to bring the My Free Birding App to bigger screens
   2023-02-16  Sven Ponelat
*/

mod library;

use crate::library::options::SettingsText;
use crate::library::tauri_supp_funcs::*;
use anyhow::{Context, Result};
use error_feedback::*;
use tauri::{CustomMenuItem, Menu, Submenu};

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

    // @@@@@@@ Showing front-end
    println!("This is from the main.rs -> Sven");
    tauri::Builder::default()
        .menu(
            Menu::new().add_submenu(Submenu::new(
                "File",
                Menu::new()
                    .add_item(CustomMenuItem::new("open", "Open").accelerator("cmdOrControl+O"))
                    .add_item(CustomMenuItem::new("save", "Save").accelerator("cmdOrControl+S"))
                    .add_item(CustomMenuItem::new("close", "Close").accelerator("cmdOrControl+Q")),
            )),
        )
        .on_menu_event(|event| match event.menu_item_id() {
            "save" => {
                let _ = event.window().emit("menu-event", "save-event").unwrap();
                // success
            }
            "open" => {
                let _ = event.window().emit("menu-event", "open-event").unwrap();
                // success
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![incoming_button_bar, init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
