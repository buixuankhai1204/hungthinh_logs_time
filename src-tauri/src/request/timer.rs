use diesel::AsChangeset;

#[derive(AsChangeset)]
#[diesel(table_name = crate::model::schema::timers)]
pub struct TimerUpdate {
    pub id: i64,
    pub player_id: Option<i64>,
    pub game_id: Option<i64>,
    pub time_each_block: Option<f32>,
}