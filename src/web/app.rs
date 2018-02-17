use rocket::Rocket;
use super::members;
use VERSION;

pub fn mount_app(rocket: Rocket) -> Rocket {
    rocket
        .mount("/", routes![version])
        .mount("/members", routes![members::hello])
}

#[get("/version")]
pub fn version() -> String {
    VERSION.to_string()
}
