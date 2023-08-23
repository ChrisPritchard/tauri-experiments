// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_info])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_info() -> String {
  let args: Vec<_> = env::args().collect();
  format!("{:?}", args)
}