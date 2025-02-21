use actix_web::{web, HttpResponse, Responder};
use paperclip::actix::*;
use serde::Deserialize;


#[derive(Deserialize, Apiv2Schema)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub skill_level: Option<String>,
}

#[api_v2_operation]
pub async fn register(item: web::Json<RegisterInput>) -> impl Responder {
    // Insert logic to save the new user into the database.
    HttpResponse::Created().json("User registered")
}

#[derive(Deserialize, Apiv2Schema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[api_v2_operation]
pub async fn login(item: web::Json<LoginInput>) -> impl Responder {
    // Implement your authentication logic here.
    HttpResponse::Ok().json("Login successful")
}
