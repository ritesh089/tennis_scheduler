use actix_web::web;

mod auth;
mod appointments;
mod leagues;
mod players;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/register").route(web::post().to(auth::register)))
            .service(web::resource("/login").route(web::post().to(auth::login)))
            // Configure additional endpoints:
            .configure(players::init_routes)
            .configure(appointments::init_routes)
            .configure(leagues::init_routes)
    );
}
