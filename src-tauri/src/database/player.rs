use crate::establish_connection;
use crate::model::player::Player;
use crate::request::player::PlayerUpdate;
use diesel::prelude::*;
use crate::model::schema::players::dsl::players;
use diesel::{insert_into, update};
use crate::model::schema::players::{age, id};

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

pub async fn get_all_players() -> Vec<Player> {
    let connection = &mut establish_connection();

    let other_players = players.filter(age.gt(18)).limit(20).select(Player::as_select()).load::<Player>(connection).expect("Can not get all players");
    other_players
}