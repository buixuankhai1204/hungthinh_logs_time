use crate::model::team::Team;
use diesel::prelude::*;
use crate::model::schema::teams::dsl::teams;
use diesel::{insert_into, update};
use crate::model::player::Player;
use crate::model::schema::establish_connection;
use crate::model::schema::teams::{id};
// use crate::request::team::TeamUpdate;

#[tauri::command]
pub async fn create_new_team(team: Team) -> Team {
    let mut connection = establish_connection();
    insert_into(teams).values(&team).execute(&mut connection).expect("Can not inser new team");
    team
}

// pub async fn update_team(team: TeamUpdate) -> TeamUpdate {
//     let mut connection = establish_connection();
//     let _ = update(teams).filter(id.eq(&team.id)).set(&team).execute(&mut connection);
//     team
// }
//
#[tauri::command]
pub async fn get_all_teams() -> Vec<Team> {
    let connection = &mut establish_connection();

    let other_teams: Vec<Team> = teams.limit(10).select(Team::as_select()).load(connection).expect("Can not get all teams");
    println!("{:?}", other_teams);
    other_teams
}

pub async fn get_team_by_id(id_input: i64) -> Option<Team> {
    let connection = &mut establish_connection();
    let team = teams.filter(id.eq(&id_input)).first::<Team>(connection).expect("Can not get team by id");
    Some(team)
}