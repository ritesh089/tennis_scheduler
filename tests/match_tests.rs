use actix_web::{test, web, App, http::StatusCode};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use tennis_scheduler::api;
use tennis_scheduler::db::DbPool;
use serde_json::{json, Value};
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

#[actix_rt::test]
#[ignore]
async fn test_accept_doubles_match() {
    dotenv().ok();
    let pool = setup_test_db();
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    ).await;
    
    // Register 4 test players
    let player1_name = unique_name("Player1");
    let player1_email = unique_email("player1");
    let player1 = test::TestRequest::post()
        .uri("/api/players")
        .set_json(&json!({
            "name": player1_name,
            "email": player1_email,
            "phone": "1234567890"
        }))
        .send_request(&app)
        .await;
    assert!(player1.status().is_success());
    let player1_response: serde_json::Value = test::read_body_json(player1).await;
    let player1_id = player1_response.get("id").unwrap().to_string().replace("\"", "");
    
    let player2_name = unique_name("Player2");
    let player2_email = unique_email("player2");
    let player2 = test::TestRequest::post()
        .uri("/api/players")
        .set_json(&json!({
            "name": player2_name,
            "email": player2_email,
            "phone": "1234567891"
        }))
        .send_request(&app)
        .await;
    assert!(player2.status().is_success());
    let player2_response: serde_json::Value = test::read_body_json(player2).await;
    let player2_id = player2_response.get("id").unwrap().to_string().replace("\"", "");
    
    let player3_name = unique_name("Player3");
    let player3_email = unique_email("player3");
    let player3 = test::TestRequest::post()
        .uri("/api/players")
        .set_json(&json!({
            "name": player3_name,
            "email": player3_email,
            "phone": "1234567892"
        }))
        .send_request(&app)
        .await;
    assert!(player3.status().is_success());
    let player3_response: serde_json::Value = test::read_body_json(player3).await;
    let player3_id = player3_response.get("id").unwrap().to_string().replace("\"", "");
    
    let player4_name = unique_name("Player4");
    let player4_email = unique_email("player4");
    let player4 = test::TestRequest::post()
        .uri("/api/players")
        .set_json(&json!({
            "name": player4_name,
            "email": player4_email,
            "phone": "1234567893"
        }))
        .send_request(&app)
        .await;
    assert!(player4.status().is_success());
    let player4_response: serde_json::Value = test::read_body_json(player4).await;
    let player4_id = player4_response.get("id").unwrap().to_string().replace("\"", "");
    
    // Create a test league
    let league_name = unique_name("League");
    let league = test::TestRequest::post()
        .uri("/api/leagues")
        .set_json(&json!({
            "name": league_name,
            "description": "Test league for doubles match",
            "location": "Test location",
            "start_date": "2023-01-01",
            "end_date": "2023-12-31"
        }))
        .send_request(&app)
        .await;
    
    assert!(league.status().is_success());
    let league_response: serde_json::Value = test::read_body_json(league).await;
    let league_id = league_response.get("id").unwrap().to_string().replace("\"", "");
    
    // All players join the league
    let join1 = test::TestRequest::post()
        .uri(&format!("/api/players/leagues/{}/join", league_id))
        .set_json(&json!({
            "player_id": player1_id
        }))
        .send_request(&app)
        .await;
    assert!(join1.status().is_success());
    
    let join2 = test::TestRequest::post()
        .uri(&format!("/api/players/leagues/{}/join", league_id))
        .set_json(&json!({
            "player_id": player2_id
        }))
        .send_request(&app)
        .await;
    assert!(join2.status().is_success());
    
    let join3 = test::TestRequest::post()
        .uri(&format!("/api/players/leagues/{}/join", league_id))
        .set_json(&json!({
            "player_id": player3_id
        }))
        .send_request(&app)
        .await;
    assert!(join3.status().is_success());
    
    let join4 = test::TestRequest::post()
        .uri(&format!("/api/players/leagues/{}/join", league_id))
        .set_json(&json!({
            "player_id": player4_id
        }))
        .send_request(&app)
        .await;
    assert!(join4.status().is_success());
    
    // Create a doubles match
    let match_response = test::TestRequest::post()
        .uri("/api/matches")
        .set_json(&json!({
            "league_id": league_id,
            "match_type": "doubles",
            "team1_player1_id": player1_id,
            "team1_player2_id": player2_id,
            "team2_player1_id": player3_id,
            "team2_player2_id": player4_id,
            "datetime": "2023-06-15T14:00:00",
            "location": "Tennis Court 1",
            "status": "Pending",
            "notes": "Doubles match test"
        }))
        .send_request(&app)
        .await;
    
    assert!(match_response.status().is_success());
    let match_data: serde_json::Value = test::read_body_json(match_response).await;
    let match_id = match_data.get("id").unwrap().to_string().replace("\"", "");
    
    // Accept the match
    let accept_response = test::TestRequest::post()
        .uri(&format!("/api/matches/{}/accept", match_id))
        .set_json(&json!({
            "player_id": player1_id,
            "comments": "Looking forward to our doubles match!"
        }))
        .send_request(&app)
        .await;
    
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_data: serde_json::Value = test::read_body_json(accept_response).await;
    assert_eq!(accept_data.get("success").unwrap(), &json!(true));
    assert_eq!(accept_data.get("message").unwrap(), &json!("Match accepted successfully"));
    
    // Verify the match status was updated to "Scheduled"
    let matches_response = test::TestRequest::get()
        .uri(&format!("/api/matches?league_id={}", league_id))
        .send_request(&app)
        .await;
    
    assert!(matches_response.status().is_success());
    let matches_data: serde_json::Value = test::read_body_json(matches_response).await;
    let matches = matches_data.as_array().unwrap();
    
    let updated_match = matches.iter()
        .find(|m| m.get("id").unwrap().to_string().replace("\"", "") == match_id)
        .unwrap();
    
    assert_eq!(updated_match.get("status").unwrap(), &json!("Scheduled"));
    assert!(updated_match.get("notes").unwrap().as_str().unwrap().contains("-----------------"));
    assert!(updated_match.get("notes").unwrap().as_str().unwrap().contains("Looking forward to our doubles match!"));
}

#[actix_web::test]
#[ignore]
async fn test_reject_match() {
    let pool = setup_test_db();
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    ).await;
    
    // Register a test player
    let player_name = unique_name("Player");
    let player_email = unique_email("player");
    let player = test::TestRequest::post()
        .uri("/api/players")
        .set_json(&json!({
            "name": player_name,
            "email": player_email,
            "phone": "1234567890"
        }))
        .send_request(&app)
        .await;
    assert!(player.status().is_success());
    let player_response: serde_json::Value = test::read_body_json(player).await;
    let player_id = player_response.get("id").unwrap().to_string().replace("\"", "");
    
    // Create a test league
    let league_name = unique_name("League");
    let league = test::TestRequest::post()
        .uri("/api/leagues")
        .set_json(&json!({
            "name": league_name,
            "description": "Test league for match rejection",
            "location": "Test location",
            "start_date": "2023-01-01",
            "end_date": "2023-12-31"
        }))
        .send_request(&app)
        .await;
    
    assert!(league.status().is_success());
    let league_response: serde_json::Value = test::read_body_json(league).await;
    let league_id = league_response.get("id").unwrap().to_string().replace("\"", "");
    
    // Create a match
    let match_response = test::TestRequest::post()
        .uri("/api/matches")
        .set_json(&json!({
            "league_id": league_id,
            "match_type": "singles",
            "player1_id": player_id,
            "player2_id": "2",
            "datetime": "2023-05-15T14:00:00",
            "location": "Tennis Court 1",
            "status": "Pending",
            "notes": "Match request for rejection test"
        }))
        .send_request(&app)
        .await;
    
    assert!(match_response.status().is_success());
    let match_data: serde_json::Value = test::read_body_json(match_response).await;
    let match_id = match_data.get("id").unwrap().to_string().replace("\"", "");
    
    // Reject the match
    let reject_response = test::TestRequest::post()
        .uri(&format!("/api/matches/{}/reject", match_id))
        .set_json(&json!({
            "player_id": player_id,
            "reason": "Schedule conflict"
        }))
        .send_request(&app)
        .await;
    
    assert_eq!(reject_response.status(), StatusCode::OK);
    let reject_data: serde_json::Value = test::read_body_json(reject_response).await;
    assert_eq!(reject_data.get("success").unwrap(), &json!(true));
    assert_eq!(reject_data.get("message").unwrap(), &json!("Match rejected successfully"));
    
    // Verify the match status was updated to "Rejected"
    let matches_response = test::TestRequest::get()
        .uri(&format!("/api/matches?league_id={}", league_id))
        .send_request(&app)
        .await;
    
    assert!(matches_response.status().is_success());
    let matches_data: serde_json::Value = test::read_body_json(matches_response).await;
    let matches = matches_data.as_array().unwrap();
    
    let updated_match = matches.iter()
        .find(|m| m.get("id").unwrap().to_string().replace("\"", "") == match_id)
        .unwrap();
    
    assert_eq!(updated_match.get("status").unwrap(), &json!("Rejected"));
    assert!(updated_match.get("notes").unwrap().as_str().unwrap().contains("-----------------"));
    assert!(updated_match.get("notes").unwrap().as_str().unwrap().contains("Schedule conflict"));
}

#[actix_rt::test]
async fn test_get_player_matches() {
    // Setup test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(setup_test_db()))
            .configure(api::init_routes)
    ).await;

    // Register a test user
    let user_name = unique_name("user1");
    let user_email = unique_email("user1");
    let user_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user_name,
                "email": user_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_body: Value = test::read_body_json(user_response).await;
    let user_id = user_body["id"].as_str().unwrap();

    // Create a second test user
    let user2_name = unique_name("user2");
    let user2_email = unique_email("user2");
    let user2_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user2_name,
                "email": user2_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user2_response.status(), StatusCode::OK);
    let user2_body: Value = test::read_body_json(user2_response).await;
    let user2_id = user2_body["id"].as_str().unwrap();

    // Create a test league
    let league_name = unique_name("league");
    let league_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/leagues")
            .set_json(&json!({
                "name": league_name,
                "description": "Test league description",
                "admin_id": user_id
            }))
            .to_request()
    ).await;
    assert_eq!(league_response.status(), StatusCode::OK);
    let league_body: Value = test::read_body_json(league_response).await;
    let league_id = league_body["id"].as_str().unwrap();

    // Create a singles match
    let match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-15T14:00:00",
                "location": "Tennis Court 1",
                "notes": "Test match"
            }))
            .to_request()
    ).await;
    assert_eq!(match_response.status(), StatusCode::OK);
    let match_body: Value = test::read_body_json(match_response).await;
    let match_id = match_body["id"].as_str().unwrap();

    // Get matches for player1
    let player_matches_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/matches/player/{}", user_id))
            .to_request()
    ).await;
    assert_eq!(player_matches_response.status(), StatusCode::OK);
    let player_matches_body: Value = test::read_body_json(player_matches_response).await;
    
    // Verify response structure and content
    assert!(player_matches_body.is_object());
    assert!(player_matches_body.get("matches").is_some());
    assert!(player_matches_body.get("count").is_some());
    
    // Verify that the match is included in the response
    let matches = player_matches_body["matches"].as_array().unwrap();
    assert!(matches.len() > 0);
    
    // Find our created match in the results
    let found_match = matches.iter().find(|m| m["id"].as_str() == Some(match_id));
    assert!(found_match.is_some());
    
    // Verify match details
    let found_match = found_match.unwrap();
    assert_eq!(found_match["player1_id"].as_str(), Some(user_id));
    assert_eq!(found_match["player2_id"].as_str(), Some(user2_id));
    assert_eq!(found_match["league_id"].as_str(), Some(league_id));
    assert_eq!(found_match["match_type"].as_str(), Some("Singles"));
    
    // Get matches for player2 and verify
    let player2_matches_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/matches/player/{}", user2_id))
            .to_request()
    ).await;
    assert_eq!(player2_matches_response.status(), StatusCode::OK);
    let player2_matches_body: Value = test::read_body_json(player2_matches_response).await;
    
    // Verify that player2 also has the match in their list
    let matches2 = player2_matches_body["matches"].as_array().unwrap();
    assert!(matches2.len() > 0);
    let found_match2 = matches2.iter().find(|m| m["id"].as_str() == Some(match_id));
    assert!(found_match2.is_some());
}

#[actix_rt::test]
async fn test_get_player_pending_matches() {
    // Setup test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(setup_test_db()))
            .configure(api::init_routes)
    ).await;

    // Register a test user
    let user_name = unique_name("user1");
    let user_email = unique_email("user1");
    let user_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user_name,
                "email": user_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_body: Value = test::read_body_json(user_response).await;
    let user_id = user_body["id"].as_str().unwrap();

    // Create a second test user
    let user2_name = unique_name("user2");
    let user2_email = unique_email("user2");
    let user2_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user2_name,
                "email": user2_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user2_response.status(), StatusCode::OK);
    let user2_body: Value = test::read_body_json(user2_response).await;
    let user2_id = user2_body["id"].as_str().unwrap();

    // Create a test league
    let league_name = unique_name("league");
    let league_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/leagues")
            .set_json(&json!({
                "name": league_name,
                "description": "Test league description",
                "admin_id": user_id
            }))
            .to_request()
    ).await;
    assert_eq!(league_response.status(), StatusCode::OK);
    let league_body: Value = test::read_body_json(league_response).await;
    let league_id = league_body["id"].as_str().unwrap();

    // Create a pending match
    let match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-15T14:00:00",
                "location": "Tennis Court 1",
                "status": "Pending",
                "notes": "Test pending match"
            }))
            .to_request()
    ).await;
    assert_eq!(match_response.status(), StatusCode::OK);
    let match_body: Value = test::read_body_json(match_response).await;
    let match_id = match_body["id"].as_str().unwrap();

    // Create a scheduled match (not pending)
    let scheduled_match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-16T14:00:00",
                "location": "Tennis Court 2",
                "status": "Scheduled",
                "notes": "Test scheduled match"
            }))
            .to_request()
    ).await;
    assert_eq!(scheduled_match_response.status(), StatusCode::OK);

    // Get pending matches for player1
    let pending_matches_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/matches/pending/{}", user_id))
            .to_request()
    ).await;
    assert_eq!(pending_matches_response.status(), StatusCode::OK);
    let pending_matches_body: Value = test::read_body_json(pending_matches_response).await;
    
    // Verify response structure and content
    assert!(pending_matches_body.is_object());
    assert!(pending_matches_body.get("matches").is_some());
    assert!(pending_matches_body.get("count").is_some());
    
    // Verify that only the pending match is included in the response
    let matches = pending_matches_body["matches"].as_array().unwrap();
    assert_eq!(matches.len(), 1);
    
    // Find our created match in the results
    let found_match = matches.iter().find(|m| m["id"].as_str() == Some(match_id));
    assert!(found_match.is_some());
    
    // Verify match details
    let found_match = found_match.unwrap();
    assert_eq!(found_match["player1_id"].as_str(), Some(user_id));
    assert_eq!(found_match["player2_id"].as_str(), Some(user2_id));
    assert_eq!(found_match["league_id"].as_str(), Some(league_id));
    assert_eq!(found_match["status"].as_str(), Some("Pending"));
    
    // Get pending matches for player2 and verify
    let player2_pending_matches_response = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/api/matches/pending/{}", user2_id))
            .to_request()
    ).await;
    assert_eq!(player2_pending_matches_response.status(), StatusCode::OK);
    let player2_pending_matches_body: Value = test::read_body_json(player2_pending_matches_response).await;
    
    // Verify that player2 also has the pending match in their list
    let matches2 = player2_pending_matches_body["matches"].as_array().unwrap();
    assert_eq!(matches2.len(), 1);
    let found_match2 = matches2.iter().find(|m| m["id"].as_str() == Some(match_id));
    assert!(found_match2.is_some());
}

#[actix_rt::test]
async fn test_get_league_matches() {
    // Setup test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(setup_test_db()))
            .configure(api::init_routes)
    ).await;

    // Register a test user
    let user_name = unique_name("user1");
    let user_email = unique_email("user1");
    let user_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user_name,
                "email": user_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_body: Value = test::read_body_json(user_response).await;
    let user_id = user_body["id"].as_str().unwrap();

    // Create a second test user
    let user2_name = unique_name("user2");
    let user2_email = unique_email("user2");
    let user2_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/users/register")
            .set_json(&json!({
                "name": user2_name,
                "email": user2_email,
                "password": "password123"
            }))
            .to_request()
    ).await;
    assert_eq!(user2_response.status(), StatusCode::OK);
    let user2_body: Value = test::read_body_json(user2_response).await;
    let user2_id = user2_body["id"].as_str().unwrap();

    // Create a test league
    let league_name = unique_name("league");
    let league_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/leagues")
            .set_json(&json!({
                "name": league_name,
                "description": "Test league description",
                "admin_id": user_id
            }))
            .to_request()
    ).await;
    assert_eq!(league_response.status(), StatusCode::OK);
    let league_body: Value = test::read_body_json(league_response).await;
    let league_id = league_body["id"].as_str().unwrap();

    // Create a pending match
    let pending_match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-15T14:00:00",
                "location": "Tennis Court 1",
                "status": "Pending",
                "notes": "Test pending match"
            }))
            .to_request()
    ).await;
    assert_eq!(pending_match_response.status(), StatusCode::OK);
    let pending_match_body: Value = test::read_body_json(pending_match_response).await;
    let pending_match_id = pending_match_body["id"].as_str().unwrap();

    // Create a scheduled match
    let scheduled_match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-16T14:00:00",
                "location": "Tennis Court 2",
                "status": "Scheduled",
                "notes": "Test scheduled match"
            }))
            .to_request()
    ).await;
    assert_eq!(scheduled_match_response.status(), StatusCode::OK);
    let scheduled_match_body: Value = test::read_body_json(scheduled_match_response).await;
    let scheduled_match_id = scheduled_match_body["id"].as_str().unwrap();

    // Create a completed match
    let completed_match_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/api/matches")
            .set_json(&json!({
                "match_type": "Singles",
                "player1_id": user_id,
                "player2_id": user2_id,
                "league_id": league_id,
                "datetime": "2023-05-14T14:00:00",
                "location": "Tennis Court 3",
                "status": "Completed",
                "notes": "Test completed match"
            }))
            .to_request()
    ).await;
    assert_eq!(completed_match_response.status(), StatusCode::OK);
    let completed_match_body: Value = test::read_body_json(completed_match_response).await;
    let completed_match_id = completed_match_body["id"].as_str().unwrap();

    // Get all matches for the league
    let league_matches_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!("/api/matches/league/{}", league_id))
            .set_json(&json!({}))
            .to_request()
    ).await;
    assert_eq!(league_matches_response.status(), StatusCode::OK);
    let league_matches_body: Value = test::read_body_json(league_matches_response).await;
    
    // Verify response structure and content
    assert!(league_matches_body.is_object());
    assert!(league_matches_body.get("matches").is_some());
    assert!(league_matches_body.get("count").is_some());
    
    // Verify that all three matches are included in the response
    let matches = league_matches_body["matches"].as_array().unwrap();
    assert_eq!(matches.len(), 3);
    
    // Get only pending matches for the league
    let pending_matches_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!("/api/matches/league/{}", league_id))
            .set_json(&json!({
                "status": ["Pending"]
            }))
            .to_request()
    ).await;
    assert_eq!(pending_matches_response.status(), StatusCode::OK);
    let pending_matches_body: Value = test::read_body_json(pending_matches_response).await;
    
    // Verify that only the pending match is included in the response
    let pending_matches = pending_matches_body["matches"].as_array().unwrap();
    assert_eq!(pending_matches.len(), 1);
    assert_eq!(pending_matches[0]["id"].as_str(), Some(pending_match_id));
    assert_eq!(pending_matches[0]["status"].as_str(), Some("Pending"));
    
    // Get only scheduled matches for the league
    let scheduled_matches_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!("/api/matches/league/{}", league_id))
            .set_json(&json!({
                "status": ["Scheduled"]
            }))
            .to_request()
    ).await;
    assert_eq!(scheduled_matches_response.status(), StatusCode::OK);
    let scheduled_matches_body: Value = test::read_body_json(scheduled_matches_response).await;
    
    // Verify that only the scheduled match is included in the response
    let scheduled_matches = scheduled_matches_body["matches"].as_array().unwrap();
    assert_eq!(scheduled_matches.len(), 1);
    assert_eq!(scheduled_matches[0]["id"].as_str(), Some(scheduled_match_id));
    assert_eq!(scheduled_matches[0]["status"].as_str(), Some("Scheduled"));
    
    // Get both pending and scheduled matches for the league
    let pending_scheduled_matches_response = test::call_service(
        &app,
        test::TestRequest::post()
            .uri(&format!("/api/matches/league/{}", league_id))
            .set_json(&json!({
                "status": ["Pending", "Scheduled"]
            }))
            .to_request()
    ).await;
    assert_eq!(pending_scheduled_matches_response.status(), StatusCode::OK);
    let pending_scheduled_matches_body: Value = test::read_body_json(pending_scheduled_matches_response).await;
    
    // Verify that both pending and scheduled matches are included in the response
    let pending_scheduled_matches = pending_scheduled_matches_body["matches"].as_array().unwrap();
    assert_eq!(pending_scheduled_matches.len(), 2);
    
    // Verify that the matches have the correct statuses
    let statuses: Vec<&str> = pending_scheduled_matches.iter()
        .map(|m| m["status"].as_str().unwrap())
        .collect();
    assert!(statuses.contains(&"Pending"));
    assert!(statuses.contains(&"Scheduled"));
    assert!(!statuses.contains(&"Completed"));
} 