use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Identifiable,
    AsChangeset,
    Serialize,
    Debug,
    Default
)]
#[diesel(table_name = crate::model::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub id: i64,
    pub full_name: String,
    pub nick_name: String,
    pub age: i32,
    pub position: String,
}

#[tauri::command]
pub fn my_custom_command() {
    println!("I was invoked from JavaScript!");
}
