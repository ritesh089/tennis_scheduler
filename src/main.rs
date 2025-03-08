mod api;
mod config;
mod db;
mod errors; // if you add custom errors

use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use actix_web::http::header;
use config::Config;
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Establish the database connection pool
    let pool = establish_connection(&config.database_url);

    // Start the HTTP server
    // Start the HTTP server with CORS middleware
    HttpServer::new(move || {
        // Configure CORS for requests from your frontend (http://localhost:3000)
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init_routes)
    })
    .bind(config.server_addr)?
    .run()
    .await
}
