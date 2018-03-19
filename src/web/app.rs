use config::{AppConfig, VERSION};
use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::Rocket;
use web::users;

pub fn mount_app(rocket: Rocket, config: AppConfig) -> Rocket {
    // TODO: Move this to some kind of application builder
    let manager = ConnectionManager::<PgConnection>::new(config.db.url);
    let pool = Pool::new(manager).expect("Failed to create pool.");
    rocket
        .mount("/", routes![version])
        // The /members routes have moved to /users, but is here for backward compatibility
        .mount("/members", users::routes())
        .mount("/users", users::routes())
        .manage(pool)
}

#[get("/version")]
pub fn version() -> String {
    VERSION.to_string()
}
