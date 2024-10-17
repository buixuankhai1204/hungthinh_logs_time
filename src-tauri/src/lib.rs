use std::env;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
mod request;
mod model;
mod database;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
