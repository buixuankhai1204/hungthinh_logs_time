use crate::establish_connection;
use crate::request::game::GameUpdate;
use diesel::prelude::*;
use crate::model::schema::games::dsl::games;
use diesel::{insert_into, update};
use crate::model::game::Game;
use crate::model::schema::games::{id};

pub async fn create_new_match(game: Game) -> Game {
    let mut connection = establish_connection();
    insert_into(games).values(&game).execute(&mut connection).expect("Can not inser new match");

    game
}

pub async fn update_game(game: GameUpdate) -> GameUpdate {
    let mut connection = establish_connection();
    let _ = update(games).filter(id.eq(&game.id)).set(&game).execute(&mut connection);

    game
}

pub async fn get_all_games() -> Vec<Game> {
    let connection = &mut establish_connection();

    let other_games = games.limit(20).select(Game::as_select()).load::<Game>(connection).expect("Can not get all games");
    other_games
}
