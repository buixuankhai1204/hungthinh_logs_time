use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable,
    Selectable,
    Insertable,
    Identifiable,
    AsChangeset,
    Serialize,
    Debug,
    Default, PartialEq)]
#[diesel(table_name = crate::model::schema::timers)]
pub struct Timer {
    pub id: i64,
    pub player_id: i64,
    pub game_id: i64,
    pub time_each_block: f32,
}
