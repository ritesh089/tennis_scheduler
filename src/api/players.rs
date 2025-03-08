use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use paperclip::actix::*;
use crate::db::{DbPool, models::Player};
use diesel::prelude::*;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/players")
            .route("", web::get().to(get_all_players))
            .route("/search", web::get().to(search_players))
            .route("/{player_id}/calendar", web::get().to(get_calendar))
            .route("/{player_id}/role", web::patch().to(update_player_role)),
    );
}

#[api_v2_operation]
pub async fn get_all_players(pool: web::Data<DbPool>) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::players::dsl::*;

    match players.load::<Player>(conn) {
        Ok(all_players) => HttpResponse::Ok().json(all_players),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch players")
    }
}

#[api_v2_operation]
pub async fn search_players(
    query: web::Query<SearchQuery>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::players::dsl::*;
    
    let search_pattern = format!("%{}%", query.name.to_lowercase());
    
    match players
        .filter(name.ilike(&search_pattern))
        .load::<Player>(conn) 
    {
        Ok(found_players) => HttpResponse::Ok().json(found_players),
        Err(_) => HttpResponse::InternalServerError().json("Failed to search players")
    }
}

#[derive(Deserialize, Apiv2Schema)]
pub struct SearchQuery {
    name: String,
}

pub async fn get_calendar(path: web::Path<i32>) -> impl Responder {
    // Retrieve player's calendar from the database.
    HttpResponse::Ok().json("Player calendar data")
}

#[derive(Deserialize, Apiv2Schema)]
pub struct UpdateRoleInput {
    pub role: String,
}

#[api_v2_operation]
pub async fn update_player_role(
    path: web::Path<String>,
    item: web::Json<UpdateRoleInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let playername = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::players::dsl::*;

    match diesel::update(players)
        .filter(name.eq(&playername))
        .set(role.eq(&item.role))
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().json("Role updated successfully"),
        Err(error) => {
            println!("Failed to update role: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to update role")
        }
    }
}


