use diesel::prelude::*;
use crate::model::schema::timers::dsl::timers;
use diesel::{insert_into};
use diesel::dsl::{now, sum};
use crate::model::schema::establish_connection;
use crate::model::schema::timers::{game_id, player_id, time_each_block};
use crate::model::timer::{Time, Timer};

pub async fn create_new_timer(timer: Timer) -> Timer {
    let mut connection = establish_connection();
    insert_into(timers).values(&timer).execute(&mut connection).expect("Can not inser new timer");
    timer
}

pub async fn get_all_timers_by_game_id(game_id_input: i64) -> Vec<Timer> {
    let connection = &mut establish_connection();

    let other_timers = timers.filter(game_id.eq(&game_id_input)).limit(20).select(Timer::as_select()).load::<Timer>(connection).expect("Can not get all timers");
    other_timers
}

pub async fn get_sum_timers_of_player_each_game(game_id_input: i64, player_id_input: i64) -> f32 {
    let connection = &mut establish_connection();
    let sum: f32 = timers.filter(game_id.eq(game_id_input)).filter(player_id.eq(player_id_input)).select(sum(time_each_block)).first(connection).expect("Can not get sum of timers");
    sum
}

pub async fn get_sum_timers_of_player_in_time(player_id_input: i64, time: Time) -> f32 {
    let connection = &mut establish_connection();
    let sum: f32 = timers.filter(player_id.eq(player_id_input)).between(now - time, now).select(sum(time_each_block)).first(connection).expect("Can not get sum of timers");
    sum
}

pub async fn get_all_timer_by_game_id_and_player_id(game_id_input: i64, player_id_input: i64) -> Vec<Timer> {
    let connection = &mut establish_connection();
    let other_timers = timers.filter(game_id.eq(&game_id_input)).or_filter(player_id.eq(&player_id_input)).limit(20).select(Timer::as_select()).get_results::<Timer>(connection).expect("Can not get all timers by player_id and game_id");
    other_timers
}