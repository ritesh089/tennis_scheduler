use actix_web::{web, HttpResponse, Responder};

pub async fn get_calendar(path: web::Path<i32>) -> impl Responder {
    // Retrieve player's calendar from the database.
    HttpResponse::Ok().json("Player calendar data")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/players")
            .route("/{player_id}/calendar", web::get().to(get_calendar)),
    );
}
