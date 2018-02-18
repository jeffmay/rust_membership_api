use db::DB;
use db::models::User;
use db::schema::users;
use diesel;
use diesel::prelude::*;
use rocket::{Route, State};
use rocket::response::status::Accepted;
use rocket_contrib::Json;
use std::sync::Arc;

/// The list of all routes in this module.
/// Be sure to add new routes to this list or they will not be registered in the router.
pub fn routes() -> Vec<Route> {
    routes![
        hello,
        find_by_id,
        create,
    ]
}

#[get("/hello/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/<id>")]
pub fn find_by_id(db: DB, id: u32) -> Option<Json<User>> {
    None
}

#[post("/", format = "application/json", data = "<input>")]
pub fn create(db: DB, input: Json<User>) -> Result<Accepted<String>, String> {
    diesel::insert_into(users::table)
        .values(&input.0)
        .get_result::<User>(&*db).expect("damn wtf"); // expost db error?
    Ok(Accepted(Some(String::from("Thanks for the user, bud"))))
}
