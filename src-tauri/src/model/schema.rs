use std::env;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

diesel::table! {
    players (id) {
        id -> BigInt,
        team_id -> BigInt,
        full_name -> Varchar,
        nick_name -> Varchar,
        age -> Integer,
        role_position -> Varchar,
    }
}

diesel::table! {
    games (id) {
        id -> BigInt,
        stadium -> Varchar,
        s1 -> BigInt,
        s2 -> BigInt,
        date -> Varchar,
        result -> VarChar,
        is_win -> Bool
    }
}

diesel::table! {
    timers (id) {
        id -> BigInt,
        player_id -> BigInt,
        game_id -> BigInt,
        time_each_block -> Float,
    }
}

diesel::table! {
    teams (id) {
        id -> BigInt,
        name -> VarChar,
        size -> Integer,
        current_size -> Integer
    }
}

diesel::table! {
    player_team(team_id, player_id) {
        player_id -> BigInt,
        team_id -> BigInt,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
    }
}
diesel::joinable!(player_team -> players (player_id));
diesel::joinable!(player_team -> teams (team_id));

diesel::joinable!(games -> teams (s1));

diesel::joinable!(players -> teams(team_id));
diesel::allow_tables_to_appear_in_same_query!(
    teams,
    players,
    player_team
);

diesel::allow_tables_to_appear_in_same_query!(
    teams,
    games,
);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}