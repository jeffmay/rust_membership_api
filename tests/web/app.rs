use rocket;
use rocket::Rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
use rust_membership_api;

pub fn mounted_app() -> Rocket {
    let conf = rust_membership_api::config::AppConfig::from_env();        
    rust_membership_api::web::app::mount_app(rocket::ignite(), conf)
}

#[test]
fn test_mount_app() {
    let client = Client::new(mounted_app()).expect("valid rocket instance");
    let mut response = client.get("/version").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some(rust_membership_api::config::VERSION.into()));
}


#[test]
fn test_insert_user() {


    let instance = mounted_app();


    let c = Client::new(instance).unwrap();
    let req = c.post("/member")
        .body(r#"{"id": 1000, "email": "foo@blah.net"}"#)
        .header(ContentType::JSON);
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Accepted);
}
