use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use dotenv::dotenv;
use std::env;
use tennis_scheduler::db::DbPool;

/// Gets a connection to the test database
pub fn get_test_db_connection() -> String {
    dotenv().ok();
    
    // Get the test database URL from environment variables
    // You can set TEST_DATABASE_URL in your .env file
    // If not set, use DATABASE_URL with _test suffix
    let database_url = match env::var("TEST_DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let main_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            // If the main URL ends with a database name, append _test to it
            if main_url.contains('/') {
                let parts: Vec<&str> = main_url.rsplitn(2, '/').collect();
                if parts.len() == 2 {
                    format!("{}/{}_{}", parts[1], parts[0], "test")
                } else {
                    format!("{}_test", main_url)
                }
            } else {
                format!("{}_test", main_url)
            }
        }
    };
    
    database_url
}

/// Creates a connection pool for the test database
pub fn create_test_db_pool() -> (DbPool, String) {
    let database_url = get_test_db_connection();
    
    // Try to connect to the test database
    match PgConnection::establish(&database_url) {
        Ok(_) => {
            // Database exists, we can use it
            println!("Connected to test database: {}", database_url);
        },
        Err(e) => {
            // Database doesn't exist or other error
            panic!("Failed to connect to test database: {}. Please make sure the test database exists and is accessible.", e);
        }
    }
    
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    
    // Reset the database to a clean state
    reset_test_database(&pool);
    
    (pool, database_url)
}

/// Resets the test database to a clean state
fn reset_test_database(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get connection from pool");
    
    // Delete all data from tables
    let _ = sql_query("DELETE FROM league_players").execute(&mut conn);
    let _ = sql_query("DELETE FROM leagues").execute(&mut conn);
    let _ = sql_query("DELETE FROM appointments").execute(&mut conn);
    let _ = sql_query("DELETE FROM players").execute(&mut conn);
    
    // Reset sequences
    let _ = sql_query("ALTER SEQUENCE players_id_seq RESTART WITH 1").execute(&mut conn);
    let _ = sql_query("ALTER SEQUENCE leagues_id_seq RESTART WITH 1").execute(&mut conn);
    let _ = sql_query("ALTER SEQUENCE appointments_id_seq RESTART WITH 1").execute(&mut conn);
    
    // Insert test data
    let _ = sql_query("
        INSERT INTO players (name, email, password_hash, skill_level, role)
        VALUES 
        ('Test Admin', 'admin@example.com', '$2a$12$szIHeK.9mOhWtAYXHEhVm.jbPgVVZO3.Yd1heeQcWlQxLpCOzlXQa', 'advanced', 'admin'),
        ('Test Player', 'player@example.com', '$2a$12$szIHeK.9mOhWtAYXHEhVm.jbPgVVZO3.Yd1heeQcWlQxLpCOzlXQa', 'intermediate', 'player')
    ").execute(&mut conn);
    
    let _ = sql_query("
        INSERT INTO leagues (league_name, description, skill_level, is_public, created_by)
        VALUES 
        ('Test League', 'A league for testing', 'intermediate', true, 'admin@example.com')
    ").execute(&mut conn);
} 