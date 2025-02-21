use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Player {
    pub player_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct League {
    pub league_id: i32,
    pub league_name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
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
