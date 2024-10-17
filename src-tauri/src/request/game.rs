use std::time::SystemTime;
use diesel::AsChangeset;

#[derive(AsChangeset)]
#[diesel(table_name = crate::model::schema::games)]
pub struct GameUpdate {
    pub id: i64,
    pub stadium: Option<String>,
    pub s1: Option<i64>,
    pub s2: Option<i64>,
    pub is_win: Option<bool>,
    pub result: Option<String>,
    pub date: Option<String>,
}