// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod context;

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            context::get_context,
            ai::get_ai_response
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
