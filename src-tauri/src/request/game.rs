use std::time::SystemTime;
use diesel::AsChangeset;

#[derive(AsChangeset)]
#[diesel(table_name = crate::model::schema::games)]
pub struct GameUpdate {
    pub id: i64,
    pub stadium: Option<String>,
    pub competitor: Option<String>,
    pub is_win: Option<bool>,
    pub result: Option<String>,
    pub date: Option<SystemTime>,
}