
use std;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use r2d2;
use r2d2_diesel;
use rocket::Rocket;
use super::members;
use VERSION;

pub fn mount_app(rocket: Rocket) -> Rocket {
    let env_var = std::env::var("DATABASE_URL").unwrap();
    let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(env_var);
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool.");

    rocket
        .mount("/", routes![version])
        .mount("/members", members::routes())
        .manage(pool)
}

#[get("/version")]
pub fn version() -> String {
    VERSION.to_string()
}
