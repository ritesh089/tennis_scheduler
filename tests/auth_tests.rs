use actix_web::{http::StatusCode, test, App, web};
use serde_json::json;
use tennis_scheduler::api;
use tennis_scheduler::db::DbPool;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;
use uuid::Uuid;

// Helper functions to generate unique names and emails
fn unique_name(prefix: &str) -> String {
    format!("{}-{}", prefix, Uuid::new_v4())
}

fn unique_email(prefix: &str) -> String {
    format!("{}_{:.8}@example.com", prefix, Uuid::new_v4())
}

// This is an integration test that requires a real database connection
// Run these tests with `cargo test --test auth_tests -- --ignored`
// Make sure you have a test database set up with the correct schema

fn setup_test_db() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_web::test]
#[ignore]
async fn test_register() {
    // Set up the database connection
    let pool = web::Data::new(setup_test_db());
    
    // Create test app with real API and database
    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(api::init_routes)
    ).await;

    let name = unique_name("Test_User");
    let email = unique_email("register");

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&json!({
            "name": name,
            "email": email,
            "password": "password123",
            "skill_level": "intermediate"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[actix_web::test]
#[ignore]
async fn test_login() {
    // Set up the database connection
    let pool = web::Data::new(setup_test_db());
    
    // Create test app with real API and database
    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(api::init_routes)
    ).await;

    // Generate unique user data
    let user_name = unique_name("Login User");
    let user_email = unique_email("login");

    // First register a user
    let register_req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&json!({
            "name": user_name,
            "email": user_email,
            "password": "password123",
            "skill_level": "intermediate"
        }))
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    assert_eq!(register_resp.status(), StatusCode::CREATED);

    // Then login with the same credentials
    let login_req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(&json!({
            "email": user_email,
            "password": "password123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    // Print the login response body
    let body_bytes = test::read_body(login_resp).await;
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("Login response body: {}", body_str);
}

#[actix_web::test]
#[ignore]
async fn test_register_duplicate_email() {
    // Set up the database connection
    let pool = web::Data::new(setup_test_db());
    
    // Create test app with real API and database
    let app = test::init_service(
        App::new()
            .app_data(pool.clone())
            .configure(api::init_routes)
    ).await;

    // Generate unique user data
    let user_name1 = unique_name("Duplicate User 1");
    let user_name2 = unique_name("Duplicate User 2");
    let user_email = unique_email("duplicate");

    // Register first user
    let register_req1 = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&json!({
            "name": user_name1,
            "email": user_email,
            "password": "password123",
            "skill_level": "intermediate"
        }))
        .to_request();

    let register_resp1 = test::call_service(&app, register_req1).await;
    assert_eq!(register_resp1.status(), StatusCode::CREATED);

    // Try to register second user with same email
    let register_req2 = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&json!({
            "name": user_name2,
            "email": user_email,
            "password": "password123",
            "skill_level": "intermediate"
        }))
        .to_request();

    let register_resp2 = test::call_service(&app, register_req2).await;
    assert_eq!(register_resp2.status(), StatusCode::INTERNAL_SERVER_ERROR);
} 