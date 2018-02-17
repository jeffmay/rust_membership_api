use rocket;
use rocket::Rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use rust_membership_api;

pub fn mounted_app() -> Rocket {
    rust_membership_api::web::app::mount_app(rocket::ignite())
}

#[test]
fn test_mount_app() {
    let client = Client::new(mounted_app()).expect("valid rocket instance");
    let mut response = client.get("/version").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some(rust_membership_api::VERSION.into()));
}