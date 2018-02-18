use rocket;

use rocket_contrib::Json;
use r2d2::{Pool, PooledConnection};
use rocket::State;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::sync::Arc;
use super::super::DbConn;

#[derive(Serialize, Deserialize, Debug)]
pub struct Member;

#[get("/hello/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}


#[get("/<id>")]
pub fn find_by_id(db: DbConn, id: u32) -> Option<Json<Member>> {

    None
}



pub fn routes() -> Vec<rocket::Route> {
    routes![hello, find_by_id]
}
