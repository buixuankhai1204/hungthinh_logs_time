use diesel::AsChangeset;

#[derive(AsChangeset)]
#[diesel(table_name = crate::model::schema::players)]
pub struct PlayerUpdate {
    pub id: i64,
    pub full_name: Option<String>,
    pub nick_name: Option<String>,
    pub age: Option<i32>,
    pub position: Option<String>,
}