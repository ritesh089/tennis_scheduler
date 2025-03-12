use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::{models::{NewMatch, Match}, DbPool, schema::matches};
use crate::db::schema::{matches as matches_schema, player_leagues};
use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Deserialize)]
pub struct CreateMatchInput {
    pub match_type: String,
    pub player1_id: Option<String>,
    pub player2_id: Option<String>,
    pub league_id: String,
    pub team1_player1_id: Option<String>,
    pub team1_player2_id: Option<String>,
    pub team2_player1_id: Option<String>,
    pub team2_player2_id: Option<String>,
    pub datetime: String,
    pub location: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct MatchQuery {
    pub league_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct AcceptMatchInput {
    pub player_id: String,
    pub comments: Option<String>,
}

#[derive(Deserialize)]
pub struct RejectMatchInput {
    pub player_id: String,
    pub reason: Option<String>,
}

pub async fn create_match(
    pool: web::Data<DbPool>,
    match_data: web::Json<CreateMatchInput>,
) -> Result<impl Responder, AppError> {
    let new_match = NewMatch {
        match_type: match_data.match_type.clone(),
        player1_id: match_data.player1_id.clone(),
        player2_id: match_data.player2_id.clone(),
        league_id: match_data.league_id.clone(),
        team1_player1_id: match_data.team1_player1_id.clone(),
        team1_player2_id: match_data.team1_player2_id.clone(),
        team2_player1_id: match_data.team2_player1_id.clone(),
        team2_player2_id: match_data.team2_player2_id.clone(),
        datetime: match_data.datetime.clone(),
        location: match_data.location.clone(),
        score: None,
        winner_id: None,
        status: match_data.status.clone(),
        notes: match_data.notes.clone(),
    };
    
    let pool_clone = pool.clone();
    
    // Execute the database operation
    let _ = web::block(move || {
        let mut conn = pool_clone.get().map_err(|_| AppError::InternalError)?;
        
        // Execute the insert and explicitly handle the result
        match diesel::insert_into(matches::table)
            .values(&new_match)
            .execute(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::InternalError),
        }
    })
    .await
    .map_err(|e| {
        eprintln!("Error creating match: {:?}", e);
        AppError::InternalError
    })?;
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": "Match created successfully",
        "success": true
    })))
}

pub async fn accept_match(
    pool: web::Data<DbPool>,
    match_id: web::Path<i32>,
    input: web::Json<AcceptMatchInput>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let match_id = match_id.into_inner();
    let player_id = input.player_id.clone();
    let comments = input.comments.clone();
    
    let result = web::block(move || {
        let mut conn = pool_clone.get().map_err(|_| AppError::InternalError)?;
        
        // First, check if the player is in the league
        let match_details: Match = matches_schema::table
            .filter(matches_schema::id.eq(match_id))
            .first(&mut conn)
            .map_err(|_| AppError::NotFound)?;
        
        // Check if player is in the league
        let player_in_league = player_leagues::table
            .filter(player_leagues::player_id.eq(&player_id))
            .filter(player_leagues::league_id.eq(&match_details.league_id))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|_| AppError::InternalError)?;
        
        if player_in_league == 0 {
            return Err(AppError::BadRequest("Player is not in the league".into()));
        }
        
        // Check if all players are in the league based on match type
        if match_details.match_type.to_lowercase() == "singles" {
            // For singles matches, check player1_id and player2_id
            if let Some(player1_id) = &match_details.player1_id {
                let player1_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(player1_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player1_in_league == 0 {
                    return Err(AppError::BadRequest("Player 1 is not in the league".into()));
                }
            }
            
            if let Some(player2_id) = &match_details.player2_id {
                let player2_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(player2_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player2_in_league == 0 {
                    return Err(AppError::BadRequest("Player 2 is not in the league".into()));
                }
            }
        } else if match_details.match_type.to_lowercase() == "doubles" {
            // For doubles matches, check team players
            // Check team1_player1_id
            if let Some(team1_player1_id) = &match_details.team1_player1_id {
                let player_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(team1_player1_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player_in_league == 0 {
                    return Err(AppError::BadRequest(format!("Team 1 Player 1 ({}) is not in the league", team1_player1_id)));
                }
            }
            
            // Check team1_player2_id
            if let Some(team1_player2_id) = &match_details.team1_player2_id {
                let player_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(team1_player2_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player_in_league == 0 {
                    return Err(AppError::BadRequest(format!("Team 1 Player 2 ({}) is not in the league", team1_player2_id)));
                }
            }
            
            // Check team2_player1_id
            if let Some(team2_player1_id) = &match_details.team2_player1_id {
                let player_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(team2_player1_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player_in_league == 0 {
                    return Err(AppError::BadRequest(format!("Team 2 Player 1 ({}) is not in the league", team2_player1_id)));
                }
            }
            
            // Check team2_player2_id
            if let Some(team2_player2_id) = &match_details.team2_player2_id {
                let player_in_league = player_leagues::table
                    .filter(player_leagues::player_id.eq(team2_player2_id))
                    .filter(player_leagues::league_id.eq(&match_details.league_id))
                    .count()
                    .get_result::<i64>(&mut conn)
                    .map_err(|_| AppError::InternalError)?;
                
                if player_in_league == 0 {
                    return Err(AppError::BadRequest(format!("Team 2 Player 2 ({}) is not in the league", team2_player2_id)));
                }
            }
        }
        
        // Update the match status to "Scheduled" and add comments
        let updated_notes = match (match_details.notes, comments) {
            (Some(existing_notes), Some(new_comments)) => Some(format!("{}\n-----------------\nAccepted: {}", existing_notes, new_comments)),
            (None, Some(new_comments)) => Some(format!("-----------------\nAccepted: {}", new_comments)),
            (Some(existing_notes), None) => Some(format!("{}\n-----------------\nAccepted by player {}", existing_notes, player_id)),
            (None, None) => Some(format!("-----------------\nAccepted by player {}", player_id)),
        };
        
        diesel::update(matches_schema::table)
            .filter(matches_schema::id.eq(match_id))
            .set((
                matches_schema::status.eq("Scheduled"),
                matches_schema::notes.eq(updated_notes),
            ))
            .execute(&mut conn)
            .map_err(|_| AppError::InternalError)?;
        
        Ok(())
    })
    .await
    .map_err(|e| {
        eprintln!("Error accepting match: {:?}", e);
        AppError::InternalError
    })?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Match accepted successfully",
        "success": true
    })))
}

pub async fn reject_match(
    pool: web::Data<DbPool>,
    match_id: web::Path<i32>,
    input: web::Json<RejectMatchInput>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let match_id = match_id.into_inner();
    let player_id = input.player_id.clone();
    let reason = input.reason.clone();
    
    let result: Result<(), AppError> = web::block(move || {
        let mut conn = pool_clone.get().map_err(|_| AppError::InternalError)?;
        
        // Check if the match exists
        let match_details: Match = matches_schema::table
            .filter(matches_schema::id.eq(match_id))
            .first(&mut conn)
            .map_err(|_| AppError::NotFound)?;
        
        // Update the match notes with rejection reason
        let updated_notes = match (match_details.notes, reason) {
            (Some(existing_notes), Some(rejection_reason)) => Some(format!("{}\n-----------------\nRejected: {}", existing_notes, rejection_reason)),
            (None, Some(rejection_reason)) => Some(format!("-----------------\nRejected: {}", rejection_reason)),
            (Some(existing_notes), None) => Some(format!("{}\n-----------------\nRejected by player {}", existing_notes, player_id)),
            (None, None) => Some(format!("-----------------\nRejected by player {}", player_id)),
        };
        
        // Update the match status to "Rejected"
        diesel::update(matches_schema::table)
            .filter(matches_schema::id.eq(match_id))
            .set((
                matches_schema::status.eq("Rejected"),
                matches_schema::notes.eq(updated_notes),
            ))
            .execute(&mut conn)
            .map_err(|_| AppError::InternalError)?;
        
        Ok(())
    })
    .await
    .map_err(|e| {
        eprintln!("Error rejecting match: {:?}", e);
        AppError::InternalError
    })?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Match rejected successfully",
        "success": true
    })))
}

pub async fn get_matches(
    pool: web::Data<DbPool>,
    query_params: web::Query<MatchQuery>,
) -> Result<impl Responder, AppError> {
    // For now, just return an empty array
    // In a real implementation, you would query the database
    
    let matches_vec: Vec<serde_json::Value> = Vec::new();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "matches": matches_vec,
        "count": matches_vec.len()
    })))
}

pub async fn get_player_matches(
    pool: web::Data<DbPool>,
    player_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let player_id = player_id.into_inner();
    
    // Use web::block to run the database query in a blocking thread
    let matches_result = web::block(move || -> Result<Vec<Match>, AppError> {
        let mut conn = pool_clone.get().map_err(|_| AppError::InternalError)?;
        
        matches_schema::table
            .filter(
                matches_schema::player1_id.eq(&player_id)
                .or(matches_schema::player2_id.eq(&player_id))
                .or(matches_schema::team1_player1_id.eq(&player_id))
                .or(matches_schema::team1_player2_id.eq(&player_id))
                .or(matches_schema::team2_player1_id.eq(&player_id))
                .or(matches_schema::team2_player2_id.eq(&player_id))
            )
            .load::<Match>(&mut conn)
            .map_err(|_| AppError::InternalError)
    })
    .await
    .map_err(|e| {
        eprintln!("Error fetching player matches: {:?}", e);
        AppError::InternalError
    })?;
    
    // Now matches_result is a Result<Vec<Match>, AppError>
    // We need to unwrap it to get Vec<Match>
    let matches = matches_result?;
    let count = matches.len();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "matches": matches,
        "count": count
    })))
}

pub async fn get_player_pending_matches(
    pool: web::Data<DbPool>,
    player_id: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let pool_clone = pool.clone();
    let player_id = player_id.into_inner();
    
    // Use web::block to run the database query in a blocking thread
    let matches_result = web::block(move || -> Result<Vec<Match>, AppError> {
        let mut conn = pool_clone.get().map_err(|_| AppError::InternalError)?;
        
        matches_schema::table
            .filter(
                matches_schema::status.eq("Pending")
                .and(
                    matches_schema::player1_id.eq(&player_id)
                    .or(matches_schema::player2_id.eq(&player_id))
                    .or(matches_schema::team1_player1_id.eq(&player_id))
                    .or(matches_schema::team1_player2_id.eq(&player_id))
                    .or(matches_schema::team2_player1_id.eq(&player_id))
                    .or(matches_schema::team2_player2_id.eq(&player_id))
                )
            )
            .load::<Match>(&mut conn)
            .map_err(|_| AppError::InternalError)
    })
    .await
    .map_err(|e| {
        eprintln!("Error fetching player pending matches: {:?}", e);
        AppError::InternalError
    })?;
    
    // Now matches_result is a Result<Vec<Match>, AppError>
    // We need to unwrap it to get Vec<Match>
    let matches = matches_result?;
    let count = matches.len();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "matches": matches,
        "count": count
    })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/matches")
            .route("", web::post().to(create_match))
            .route("", web::get().to(get_matches))
            .route("/player/{player_id}", web::get().to(get_player_matches))
            .route("/pending/{player_id}", web::get().to(get_player_pending_matches))
            .route("/{match_id}/accept", web::post().to(accept_match))
            .route("/{match_id}/reject", web::post().to(reject_match))
    );
}
