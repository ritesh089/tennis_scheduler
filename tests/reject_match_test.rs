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
async fn test_reject_match() {
    dotenv().ok();
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