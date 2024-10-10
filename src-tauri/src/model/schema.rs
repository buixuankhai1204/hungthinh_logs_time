diesel::table! {
    players (id) {
        id -> BigInt,
        full_name -> Varchar,
        nick_name -> Varchar,
        age -> Integer,
        position -> Varchar,
    }
}
diesel::table! {
    games (id) {
        id -> BigInt,
        stadium -> Varchar,
        competitor -> Varchar,
        date -> Timestamp,
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