use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Queryable,
    Selectable,
    Deserialize,
    Insertable,
    Identifiable,
    AsChangeset,
    Serialize,
    Debug
)]
#[diesel(table_name = crate::model::schema::games)]
#[diesel(belongs_to(crate::model::player::Team))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i64,
    pub stadium: String,
    pub s1: i64,
    pub s2: i64,
    pub date: String,
    pub result: String,
    pub is_win: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
