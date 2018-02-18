use db::DB;
use diesel;
use diesel::prelude::*;
use models::*;
use rocket::{Route, State};
use rocket_contrib::Json;
use rocket::response::status::Accepted;
use std::sync::Arc;


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

//    use schema::users::dsl::*;
    use schema::users;

    diesel::insert_into(users::table)
        .values(&input.0)
        .get_result::<User>(&*db).expect("damn wtf"); // expost db error?


    Ok(Accepted(Some(String::from("Thanks for the user, bud"))))
}

pub fn routes() -> Vec<Route> {
    routes![hello, find_by_id, create]
}
