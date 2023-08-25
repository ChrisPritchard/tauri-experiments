// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use components::City;
use legion::{World, IntoQuery};
use tauri::State;

mod components;

struct GameState {
    pub world: World
}

fn main() {

    let mut world = World::default();

    world.push(((), City { name: String::from("Wellington"), coords: (-41.2867, 174.7730) }));
    world.push(((), City { name: String::from("New York"), coords: (40.71427, -74.00597) }));
    world.push(((), City { name: String::from("Paris"), coords: (48.85341, 2.3488) }));
    world.push(((), City { name: String::from("Hong Kong"), coords: (22.396428, 114.109497) }));

    tauri::Builder::default()
        .manage(Mutex::new(GameState{ world }))
        .invoke_handler(tauri::generate_handler![get_cities, add_city])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_cities(game_state: State<Mutex<GameState>>) -> Vec<City> {
    let game_state = game_state.lock().unwrap();
    let cities: Vec<_> = <&City>::query().iter(&game_state.world).map(|c| c.clone()).collect();
    cities
}

#[tauri::command]
fn add_city(name: String, lat: f32, long: f32, game_state: State<Mutex<GameState>>) {
    let mut game_state = game_state.lock().unwrap();
    game_state.world.push(((), City { name, coords: (lat, long) }));
}