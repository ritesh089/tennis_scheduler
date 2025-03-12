use actix_web::{test, web, App, http::StatusCode};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;
use tennis_scheduler::api;
use tennis_scheduler::db::DbPool;
use serde_json::json;

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
    assert!(updated_match.get("notes").unwrap().as_str().unwrap().contains("Looking forward to our doubles match!"));
} 