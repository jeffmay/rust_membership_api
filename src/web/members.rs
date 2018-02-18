use db::DB;
use diesel::prelude::*;
use models::User;
use rocket::{Route, State};
use rocket_contrib::Json;
use std::sync::Arc;

#[get("/hello/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/<id>")]
pub fn find_by_id(db: DB, id: u32) -> Option<Json<User>> {
    None
}

pub fn routes() -> Vec<Route> {
    routes![hello, find_by_id]
}
