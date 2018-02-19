use std;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct DBConfig {
    pub url: String,
}

pub struct AppConfig {
    pub db: DBConfig,
}

impl AppConfig {

    /// Attempt to parse environment configs, panic on missing required configs.
    pub fn from_env() -> AppConfig {
        AppConfig {
            db: DBConfig {
                // TODO: Figure out generic way to fail on missing environment variables
                url: std::env::var("DATABASE_URL").expect("Environment variable DATABASE_URL"),
            },
        }
    }
}
