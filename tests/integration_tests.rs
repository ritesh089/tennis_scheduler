// tests/integration_tests.rs

use actix_web::{http::StatusCode, test, App};
use serde_json::json;
use tennis_scheduler::api;

#[actix_web::test]
async fn test_register() {
    let app = test::init_service(App::new().configure(api::init_routes)).await;

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&json!({
            "name": "Test User",
            "email": "test@example.com",
            "password": "password123",
            "skill_level": "intermediate"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[actix_web::test]
async fn test_login() {
    let app = test::init_service(App::new().configure(api::init_routes)).await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_calendar() {
    let app = test::init_service(App::new().configure(api::init_routes)).await;

    // For demonstration, we're calling player_id 1.
    let req = test::TestRequest::get()
        .uri("/api/players/1/calendar")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_create_appointment() {
    let app = test::init_service(App::new().configure(api::init_routes)).await;

    let req = test::TestRequest::post()
        .uri("/api/appointments")
        .set_json(&json!({
            "requester_id": 1,
            "opponent_id": 2,
            "start_time": "2025-03-01T10:00:00",
            "end_time": "2025-03-01T11:00:00",
            "league_id": null
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}
