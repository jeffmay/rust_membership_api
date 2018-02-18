use config::{AppConfig, VERSION};
use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::Rocket;
use web::members;

pub fn mount_app(rocket: Rocket, config: AppConfig) -> Rocket {
    // TODO: Move this to some kind of application builder
    let manager = ConnectionManager::<PgConnection>::new(config.db.url);
    let pool = Pool::new(manager).expect("Failed to create pool.");
    rocket
        .mount("/", routes![version])
        .mount("/members", members::routes())
        .manage(pool)
}

#[get("/version")]
pub fn version() -> String {
    VERSION.to_string()
}
