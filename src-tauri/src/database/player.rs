use crate::model::player::{Player};
use diesel::prelude::*;
use crate::model::schema::players::dsl::players;
use diesel::{insert_into, update};
use diesel::associations::HasTable;
use crate::model::schema::establish_connection;
use crate::model::schema::player_team::{player_id, team_id as team_join_id};
use crate::model::schema::players::{age, id};
use crate::model::schema::teams::{id as team_own_id};
use crate::model::team::{PlayerTeam, Team};
use crate::request::player::PlayerUpdate;
use diesel::BelongingToDsl;
use crate::database::team::get_team_by_id;
use crate::model::schema::player_team::dsl::player_team;
use crate::model::schema::teams::dsl::teams;

pub async fn create_new_player(player: Player) -> Player {
    let mut connection = establish_connection();
    insert_into(players).values(&player).execute(&mut connection).expect("Can not inser new player");

    player
}

pub async fn update_player(player: PlayerUpdate) -> PlayerUpdate {
    let mut connection = establish_connection();
    let _ = update(players).filter(id.eq(&player.id)).set(&player).execute(&mut connection);
    player
}

#[tauri::command]
pub async fn get_all_players() -> Vec<Player> {
    let connection = &mut establish_connection();

    let other_players = players.filter(age.gt(18)).limit(20).select(Player::as_select()).load::<Player>(connection).expect("Can not get all players");
    other_players
}

#[tauri::command]
pub async fn get_player_by_id(id_input: i64) -> Option<Player> {
    let connection = &mut establish_connection();
    let player = players.filter(id.eq(&id_input)).first::<Player>(connection).expect("Can not get player by id");
    Some(player)
}

#[tauri::command]
pub async fn get_all_player_by_team_id(team_id_input: i64) -> (Team, Vec<Player>) {
    let team = get_team_by_id(team_id_input).await.expect("can not find team by team id");
    let connection = &mut establish_connection();
    let players_res: Vec<Player> = PlayerTeam::belonging_to(&team).inner_join(Player::table()).select(Player::as_select()).load::<Player>(connection).expect("Can not get all players");
    (team, players_res)
}