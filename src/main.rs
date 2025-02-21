mod api;
mod config;
mod db;
mod errors; // if you add custom errors

use actix_web::{App, HttpServer};
use config::Config;
use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Establish the database connection pool
    let pool = establish_connection(&config.database_url);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(api::init_routes)
    })
    .bind(config.server_addr)?
    .run()
    .await
}
