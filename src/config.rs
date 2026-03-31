use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_path: String,
    pub base_url: String,
    pub code_length: usize,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            database_path: env::var("DATABASE_PATH").unwrap_or_else(|_| "urls.db".to_string()),
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            code_length: env::var("CODE_LENGTH")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(7),
        }
    }
}
