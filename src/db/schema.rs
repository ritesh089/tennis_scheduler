// @generated automatically by Diesel CLI.

use diesel_derive_enum::DbEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "Match_type_enum"]
pub enum MatchType {
    Singles,
    Doubles,
    Practice
}
#[derive(Debug, DbEnum, Serialize, Deserialize)]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
#[DieselType = "Match_status_enum"]
pub enum MatchStatus {
    Scheduled,
    InProgress, 
    Completed,
    Cancelled
}

diesel::table! {
    players (player_id) {
        player_id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        skill_level -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    matches (match_id) {
        match_id -> Int4,
        league_id -> Int4,
        match_type -> Varchar,
        match_scheduled_time -> Nullable<Timestamp>,
        match_actual_time -> Nullable<Timestamp>,
        match_location -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        match_result -> Nullable<Varchar>,
        player1_id -> Nullable<Int4>,
        player2_id -> Nullable<Int4>,
        team1_player1_id -> Nullable<Int4>,
        team1_player2_id -> Nullable<Int4>,
        team2_player1_id -> Nullable<Int4>,
        team2_player2_id -> Nullable<Int4>,
        winner_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    appointments (appointment_id) {
        appointment_id -> Int4,
        requester_id -> Int4,
        opponent_id -> Int4,
        league_id -> Nullable<Int4>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    leagues (league_id) {
        league_id -> Int4,
        league_name -> Varchar,
        description -> Nullable<Text>,
        skill_level -> Nullable<Varchar>,
        created_by -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    player_leagues (player_id, league_id) {
        player_id -> VarChar,
        league_id -> VarChar,
        role -> Varchar,
        singles_ranking -> Nullable<Int4>,
        doubles_ranking -> Nullable<Int4>,
        joined_at -> Timestamp,
    }
}
