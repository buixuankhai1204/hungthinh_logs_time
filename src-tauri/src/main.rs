 #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 mod model;
 mod database;
 mod request;
 use database::player::get_all_players;
 use database::team::{create_new_team, get_all_teams};
 use crate::database::game::create_new_game;

 #[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

fn main() {
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_all_players, create_new_team, get_all_teams, create_new_game])
   .run(tauri::generate_context!())
   .expect("error while running tauri application");
}