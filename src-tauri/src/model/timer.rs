use chrono::NaiveDateTime;
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
    pub team_id: i64,
    pub time_each_block: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum Time {
    Week = 604800,
    Month = 2419200,
}