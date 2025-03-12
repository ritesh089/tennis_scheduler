use actix_web::web;

mod auth;
mod appointments;
mod leagues;
mod players;
mod matches;

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
                    .route("/{league_id}", web::get().to(leagues::get_league_by_name))
                    .route("/{league_id}/join", web::post().to(leagues::join_league))
                    .route("/{league_id}/join-requests", web::post().to(leagues::create_join_request))
                    .route("/{league_id}/join-requests", web::get().to(leagues::get_league_join_requests))
                    .route("/{league_id}/join-requests/{request_id}", web::patch().to(leagues::update_join_request_status))
                    .service(
                        web::resource("/{league_id}/players")
                            .route(web::get().to(leagues::get_league_players))
                    )
                    .service(
                        web::resource("/{league_id}/members/{player_id}")
                            .route(web::put().to(leagues::update_member_role))
                    )
                    .route("/{league_id}/players/{player_name}/role", web::get().to(leagues::get_player_league_role)),
            )
            // Other endpoints...
            .configure(players::init_routes)
            .configure(appointments::init_routes)
            .configure(matches::init_routes)
    );
}
