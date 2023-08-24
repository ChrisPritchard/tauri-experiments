// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, sync::Mutex};

use tauri::{Window, State};

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(GameState { counter: 0 }))
        .invoke_handler(tauri::generate_handler![get_info, inc_count])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_info() -> String {
    let args: Vec<_> = env::args().collect();
    format!("{:?}", args)
}

struct GameState {
    counter: i64
}

#[tauri::command]
fn inc_count(window: Window, state: State<Mutex<GameState>>) {
    (*state.lock().unwrap()).counter += 1;
    let _ = window.emit("counter_update", state.lock().unwrap().counter);
}