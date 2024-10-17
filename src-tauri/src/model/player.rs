use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use crate::model::team::Team;

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
#[diesel(belongs_to(crate::model::player::Team))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub id: i64,
    pub team_id: i64,
    pub full_name: String,
    pub nick_name: String,
    pub age: i32,
    pub role_position: String,

}
