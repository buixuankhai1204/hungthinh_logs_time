use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Default
)]
#[diesel(table_name = crate::model::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub size: i32,
    pub current_size: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[diesel(belongs_to(crate::model::team::Team))]
#[diesel(belongs_to(crate::model::player::Player))]
#[diesel(table_name = crate::model::schema::player_team)]
#[diesel(primary_key(team_id, player_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlayerTeam {
    player_id: i64,
    team_id: i64,
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
}