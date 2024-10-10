use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Serialize, Serializer};
use std::time::SystemTime;

#[derive(Queryable,
    Selectable,
    Insertable,
    Identifiable,
    AsChangeset,
    Serialize,
    Debug
)]
#[diesel(table_name = crate::model::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i64,
    pub stadium: String,
    pub competitor: String,
    pub date: SystemTime,
    pub result: String,
    pub is_win: bool,
}
