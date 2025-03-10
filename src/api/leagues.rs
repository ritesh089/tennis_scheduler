use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use paperclip::actix::*;
use crate::db::{models::{League, NewLeague, LeagueJoinRequest, NewLeagueJoinRequest}, DbPool};
use chrono::Local;
use diesel::prelude::*;

use crate::db::schema::leagues::dsl::{
    leagues as all_leagues,
    league_name
};
use crate::db::schema::player_leagues::dsl::{
    player_leagues as all_player_leagues, 
    player_id, 
    role, 
    joined_at,
    league_id
    
};
use crate::db::models::Player;

#[derive(Debug, Deserialize)]
pub struct CreateLeagueInput {
    pub league_name: String,
    pub description: Option<String>,
    pub skill_level: Option<String>,
    pub created_by: String,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LeagueResponse {
    pub league_id: i32,
    pub league_name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn get_league_by_id(
    path: web::Path<i32>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_one = path.into_inner();
    
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match all_leagues.find(league_id_one).first::<League>(conn) {
        Ok(league) => HttpResponse::Ok().json(league),
        Err(_) => HttpResponse::NotFound().json("League not found")
    }
}

pub async fn get_league_by_name(
    path: web::Path<String>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_name_val = path.into_inner();
    
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match all_leagues
        .filter(league_name.eq(league_name_val))
        .first::<League>(conn) 
    {
        Ok(league) => HttpResponse::Ok().json(league),
        Err(_) => HttpResponse::NotFound().json("League not found")
    }
}


pub async fn create_league(
    item: web::Json<CreateLeagueInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    // Start a transaction
    conn.transaction(|conn| {
        // Insert the league
        let new_league = NewLeague {
            league_name: item.league_name.to_string(),
            description: item.description.clone(),
            skill_level: item.skill_level.clone(),
            created_by: item.created_by.to_string(),
            is_public: item.is_public.unwrap_or(true),
            created_at: Local::now().naive_local(),
        };

        let league_result = diesel::insert_into(all_leagues)
            .values(&new_league)
            .get_result::<League>(conn)?;

        // Add creator as admin
        diesel::insert_into(all_player_leagues)
            .values((
                player_id.eq(&item.created_by),
                league_id.eq(&league_result.league_name.to_string()),
                role.eq("admin"),
           
                joined_at.eq(Local::now().naive_local())
            ))
            .execute(conn)?;

        Ok::<_, diesel::result::Error>(league_result)
    })
    .map(|league| HttpResponse::Created().json(league))
    .unwrap_or_else(|_| HttpResponse::InternalServerError().json("Failed to create league"))
}

#[derive(Debug, Deserialize)]
pub struct JoinLeagueInput {
    pub player_id: String,
}

pub async fn join_league(
    path: web::Path<String>,
    item: web::Json<JoinLeagueInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_val = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match diesel::insert_into(all_player_leagues)
        .values((
            player_id.eq(item.player_id.to_string()),
            league_id.eq(league_id_val),
            role.eq("player"),
            joined_at.eq(Local::now().naive_local())
        ))
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().json("Joined league successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to join league")
    }
}

pub async fn leave_league(
    path: web::Path<String>,
    item: web::Json<JoinLeagueInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_val = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    match diesel::delete(all_player_leagues)
        .filter(player_id.eq(item.player_id.to_string()))
        .filter(league_id.eq(league_id_val))
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().json("Left league successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to leave league")
    }
}



#[derive(Debug, Deserialize)]
pub struct LeagueQuery {
    search: Option<String>,
}

pub async fn search_leagues(
    query: web::Query<LeagueQuery>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let mut query_builder = all_leagues.into_boxed();
    
    if let Some(search_term) = &query.search {
        query_builder = query_builder.filter(league_name.ilike(format!("%{}%", search_term)));
    }

    match query_builder.load::<League>(conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(error) => {
            println!("Failed to search leagues");
            println!("Error: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to search leagues")
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct UpdateRoleInput {
    pub role: String, // e.g., "admin" or "player"
}

pub async fn update_member_role(
    path: web::Path<(String, String)>,
    item: web::Json<UpdateRoleInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let (league_id_val, player_id_val) = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");
    
    use crate::db::schema::player_leagues;
    match diesel::update(player_leagues::table)
        .filter(player_leagues::league_id.eq(&league_id_val))
        .filter(player_leagues::player_id.eq(&player_id_val))
        .set(player_leagues::role.eq(&item.role))
        .execute(conn) 
    {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to update member role")
    }
    
    HttpResponse::Ok().json(format!("Updated player {} in league {} to role {}", player_id_val, league_id_val, item.role))
}

#[derive(Serialize, Queryable, Debug)]
pub struct LeaguePlayerInfo {
    pub name: String,
    pub email: String,
    pub skill_level: Option<String>,
    pub role: String,
}

#[api_v2_operation]
pub async fn get_league_players(
    path: web::Path<String>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_val = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::{players, player_leagues};
    
    match players::table
        .inner_join(player_leagues::table.on(
            players::name.eq(player_leagues::player_id)
            .and(player_leagues::league_id.eq(league_id_val))
        ))
        .select((
            players::name,
            players::email,
            players::skill_level,
            player_leagues::role,
        ))
        .load::<LeaguePlayerInfo>(conn)
    {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(error) => {
            println!("Failed to fetch league players: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to fetch league players")
        }
    }
}

#[derive(Serialize)]
pub struct PlayerLeagueRole {
    pub role: String,
}

#[api_v2_operation]
pub async fn get_player_league_role(
    path: web::Path<(String, String)>,  // (league_id, player_name)
    pool: web::Data<DbPool>
) -> impl Responder {
    let (league_id_val, player_name) = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::player_leagues::dsl::*;
    
    match player_leagues
        .filter(league_id.eq(league_id_val))
        .filter(player_id.eq(player_name))
        .select(role)
        .first::<String>(conn)
    {
        Ok(player_role) => HttpResponse::Ok().json(PlayerLeagueRole { role: player_role }),
        Err(_) => HttpResponse::NotFound().json("Player not found in this league")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoinRequestInput {
    pub player_id: String,
    pub description: Option<String>,
}

pub async fn create_join_request(
    path: web::Path<String>,
    item: web::Json<JoinRequestInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_val = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::league_join_requests::dsl::*;

    let new_request = NewLeagueJoinRequest {
        league_id: league_id_val,
        player_id: item.player_id.clone(),
        description: item.description.clone(),
    };

    match diesel::insert_into(league_join_requests)
        .values(&new_request)
        .execute(conn)
    {
        Ok(_) => HttpResponse::Created().json("Join request created successfully"),
        Err(error) => {
            println!("Failed to create join request: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to create join request")
        }
    }
}

pub async fn get_league_join_requests(
    path: web::Path<String>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let league_id_val = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::league_join_requests::dsl::*;

    match league_join_requests
        .filter(league_id.eq(league_id_val))
        .filter(status.eq("pending"))
        .load::<LeagueJoinRequest>(conn)
    {
        Ok(requests) => HttpResponse::Ok().json(requests),
        Err(error) => {
            println!("Failed to fetch join requests: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to fetch join requests")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateJoinRequestInput {
    pub status: String,  // "accepted" or "rejected"
}


pub async fn update_join_request_status(
    path: web::Path<(String, i32)>,  // (league_id, request_id)
    item: web::Json<UpdateJoinRequestInput>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let (league_id_val, request_id_val) = path.into_inner();
    let conn = &mut pool.get().expect("Failed to get DB connection");

    use crate::db::schema::league_join_requests::dsl::*;

    match diesel::update(league_join_requests)
        .filter(league_id.eq(league_id_val))
        .filter(request_id.eq(request_id_val))
        .set(status.eq(&item.status))
        .execute(conn)
    {
        Ok(_) => HttpResponse::Ok().json("Join request status updated successfully"),
        Err(error) => {
            println!("Failed to update join request status: {:?}", error);
            HttpResponse::InternalServerError().json("Failed to update join request status")
        }
    }
}




