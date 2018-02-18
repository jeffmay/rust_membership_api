use config::{DATABASE_URL, VERSION};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use rocket::Rocket;
use std;
use web::members;

pub fn mount_app(rocket: Rocket) -> Rocket {
    // TODO: Move this to some kind of application builder
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
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
