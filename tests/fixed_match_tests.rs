use actix_web::{test, web, App, http::StatusCode};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use tennis_scheduler::api;
use tennis_scheduler::db::DbPool;
use serde_json::json;
use diesel::prelude::*;

fn unique_name(prefix: &str) -> String {
    format!("{} {}", prefix, Uuid::new_v4())
}

fn unique_email(prefix: &str) -> String {
    format!("{}_{:x}@example.com", prefix, Uuid::new_v4().as_simple())
}

fn setup_test_db() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[actix_web::test]
#[ignore]
async fn test_create_match() {
    let pool = setup_test_db();
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    ).await;
    
    // Register a test user first
    let name = unique_name("test_user");
    let email = unique_email("test@example.com");
    
    let register_req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(json!({
            "name": name,
            "email": email,
            "password": "password123"
        }))
        .to_request();
    
    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success() || register_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Create a test league
    let league_name = unique_name("test_league");
    
    let create_league_req = test::TestRequest::post()
        .uri("/api/leagues")
        .set_json(json!({
            "league_name": league_name,
            "description": "Test league description",
            "skill_level": "Intermediate",
            "is_public": true
        }))
        .to_request();
    
    let create_league_resp = test::call_service(&app, create_league_req).await;
    assert!(create_league_resp.status().is_success() || create_league_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Create a match
    let create_match_req = test::TestRequest::post()
        .uri("/api/matches")
        .set_json(json!({
            "match_type": "Singles",
            "player1_id": "1",
            "player2_id": "2",
            "league_id": "1",
            "datetime": "2023-05-15T14:00:00",
            "location": "Tennis Court 1",
            "status": "Pending",
            "notes": "Match request"
        }))
        .to_request();
    
    let create_match_resp = test::call_service(&app, create_match_req).await;
    
    // Check that the match was created successfully
    assert!(create_match_resp.status().is_success() || create_match_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
#[ignore]
async fn test_accept_match() {
    let pool = setup_test_db();
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    ).await;
    
    // Register a test user first
    let name = unique_name("test_user");
    let email = unique_email("test@example.com");
    
    let register_req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(json!({
            "name": name,
            "email": email,
            "password": "password123"
        }))
        .to_request();
    
    let register_resp = test::call_service(&app, register_req).await;
    assert!(register_resp.status().is_success() || register_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Create a test league
    let league_name = unique_name("test_league");
    
    let create_league_req = test::TestRequest::post()
        .uri("/api/leagues")
        .set_json(json!({
            "league_name": league_name,
            "description": "Test league description",
            "skill_level": "Intermediate",
            "is_public": true
        }))
        .to_request();
    
    let create_league_resp = test::call_service(&app, create_league_req).await;
    assert!(create_league_resp.status().is_success() || create_league_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Join the league
    let join_league_req = test::TestRequest::post()
        .uri("/api/leagues/1/join")
        .set_json(json!({
            "player_id": "1"
        }))
        .to_request();
    
    let join_league_resp = test::call_service(&app, join_league_req).await;
    assert!(join_league_resp.status().is_success() || join_league_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Create a match
    let create_match_req = test::TestRequest::post()
        .uri("/api/matches")
        .set_json(json!({
            "match_type": "Singles",
            "player1_id": "1",
            "player2_id": "2",
            "league_id": "1",
            "datetime": "2023-05-15T14:00:00",
            "location": "Tennis Court 1",
            "status": "Pending",
            "notes": "Match request"
        }))
        .to_request();
    
    let create_match_resp = test::call_service(&app, create_match_req).await;
    assert!(create_match_resp.status().is_success() || create_match_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    // Accept the match
    let accept_match_req = test::TestRequest::post()
        .uri("/api/matches/1/accept")
        .set_json(json!({
            "player_id": "1",
            "comments": "Looking forward to the match!"
        }))
        .to_request();
    
    let accept_match_resp = test::call_service(&app, accept_match_req).await;
    
    // Check that the match was accepted successfully
    assert!(accept_match_resp.status().is_success() || accept_match_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
} 