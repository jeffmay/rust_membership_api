
pub const DATABASE_URL: &'static str = option_env!("DATABASE_URL")
    .expect("DATABASE_URL is not defined");

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
