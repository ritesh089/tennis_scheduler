use std::env;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let server_addr = env::var("SERVER_ADDR")?;
        Ok(Config { database_url, server_addr })
    }
}
