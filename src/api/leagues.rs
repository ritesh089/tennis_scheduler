use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateLeagueInput {
    pub league_name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct LeagueResponse {
    pub league_id: i32,
    pub league_name: String,
}

pub async fn create_league(item: web::Json<CreateLeagueInput>) -> impl Responder {
    // Insert your database logic to create a league.
    let response = LeagueResponse {
        league_id: 1,
        league_name: item.league_name.clone(),
    };
    HttpResponse::Created().json(response)
}

#[derive(Deserialize)]
pub struct JoinLeagueInput {
    pub player_id: i32,
}

pub async fn join_league(
    path: web::Path<i32>,
    item: web::Json<JoinLeagueInput>,
) -> impl Responder {
    // Insert your logic for joining a league.
    HttpResponse::Ok().json("Joined league successfully")
}

pub async fn leave_league(
    path: web::Path<i32>,
    item: web::Json<JoinLeagueInput>,
) -> impl Responder {
    // Insert your logic for leaving a league.
    HttpResponse::Ok().json("Left league successfully")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/leagues")
            .route("", web::post().to(create_league))
            .route("/{league_id}/join", web::post().to(join_league))
            .route("/{league_id}/leave", web::delete().to(leave_league)),
    );
}
