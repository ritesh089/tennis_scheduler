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
            // League endpoints:
            .service(
                web::scope("/leagues")
                    .route("", web::get().to(leagues::search_leagues))
                    .route("", web::post().to(leagues::create_league))
                    .route("/{league_id}", web::get().to(leagues::get_league_by_id))
                    .service(
                        web::resource("/{league_id}/join").route(web::post().to(leagues::join_league))
                    )
                    .service(
                        web::resource("/{league_id}/members")
                            .route(web::get().to(leagues::get_league_members))
                    )
                    .service(
                        web::resource("/{league_id}/members/{player_id}")
                            .route(web::put().to(leagues::update_member_role))
                    )
            )
            // Other endpoints...
            .configure(players::init_routes)
            .configure(appointments::init_routes)
    );
}
