use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Player {
    pub player_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::leagues)]
pub struct League {
    pub league_id: i32,
    pub league_name: String,
    pub skill_level: Option<String>,
    pub is_public: bool,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct LeaguePlayer {
    pub league_id: i32,
    pub player_id: i32,
    pub singles_ranking: Option<i32>,
    pub doubles_ranking: Option<i32>,
    pub role: String,  // app role: "admin", "user", etc.
    pub league_role: String,  // league role: "manager" or "player"
    pub joined_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Appointment {
    pub appointment_id: i32,
    pub requester_id: i32,
    pub opponent_id: i32,
    pub league_id: Option<i32>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::leagues)]
pub struct NewLeague {
    pub league_name: String,
    pub description: Option<String>,
    pub skill_level: Option<String>,
    pub created_by: String,
    pub is_public: bool,
    pub created_at: chrono::NaiveDateTime,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct Match {
    pub match_id: i32,
    pub league_id: i32,
    pub match_type: String,
    pub match_scheduled_time: Option<chrono::NaiveDateTime>,
    pub match_actual_time: Option<chrono::NaiveDateTime>,
    pub match_location: Option<String>,
    pub status: String,
    pub match_result: Option<String>,
    pub player1_id: Option<i32>,
    pub player2_id: Option<i32>,
    pub team1_player1_id: Option<i32>,
    pub team1_player2_id: Option<i32>,
    pub team2_player1_id: Option<i32>,
    pub team2_player2_id: Option<i32>,
    pub winner_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct LeagueJoinRequest {
    pub request_id: i32,
    pub league_id: String,
    pub player_id: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::league_join_requests)]
pub struct NewLeagueJoinRequest {
    pub league_id: String,
    pub player_id: String,
    pub description: Option<String>,
}





