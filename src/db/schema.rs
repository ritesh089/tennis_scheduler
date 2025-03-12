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
        role -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    matches (id) {
        id -> Int4,
        match_type -> VarChar,
        player1_id -> Nullable<Text>,
        player2_id -> Nullable< Text>,
        league_id -> Text,
        team1_player1_id -> Nullable<Text>,
        team1_player2_id -> Nullable<Text>,
        team2_player1_id -> Nullable<Text>,
        team2_player2_id -> Nullable<Text>,
        datetime -> Text,
        location -> Text,
        score -> Nullable<Text>,
        winner_id -> Nullable<Text>,
        status -> Text,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
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
        is_public -> Bool,
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

diesel::table! {
    league_join_requests (request_id) {
        request_id -> Int4,
        league_id -> Varchar,
        player_id -> Varchar,
        description -> Nullable<Text>,
        status -> Varchar,
        created_at -> Timestamp,
        notes -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    players,
    player_leagues,
    leagues,
    matches,
    appointments,
    league_join_requests,
);
