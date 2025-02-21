use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAppointmentInput {
    pub requester_id: i32,
    pub opponent_id: i32,
    pub start_time: String, // Ideally, use a datetime type; using String here for simplicity.
    pub end_time: String,
    pub league_id: Option<i32>,
}

#[derive(Serialize)]
pub struct AppointmentResponse {
    pub appointment_id: i32,
    pub status: String,
}

pub async fn create_appointment(item: web::Json<CreateAppointmentInput>) -> impl Responder {
    // Insert your database logic here.
    let response = AppointmentResponse {
        appointment_id: 1,
        status: "pending".into(),
    };
    HttpResponse::Created().json(response)
}

#[derive(Deserialize)]
pub struct UpdateAppointmentInput {
    pub status: String,
}

pub async fn update_appointment(
    path: web::Path<i32>,
    item: web::Json<UpdateAppointmentInput>,
) -> impl Responder {
    // Update appointment logic goes here.
    HttpResponse::Ok().json("Appointment updated")
}

pub async fn cancel_appointment(path: web::Path<i32>) -> impl Responder {
    // Logic to cancel the appointment goes here.
    HttpResponse::Ok().json("Appointment canceled")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/appointments")
            .route("", web::post().to(create_appointment))
            .route("/{appointment_id}", web::put().to(update_appointment))
            .route("/{appointment_id}", web::delete().to(cancel_appointment)),
    );
}
