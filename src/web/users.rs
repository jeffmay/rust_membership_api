use db::DB;
use db::models::{ID, User};
use rocket::response::status::Created;
use rocket::Route;
use rocket_contrib::Json;
use service::rest;
use service::rest::models::NewUser;
use service::rest::result::ApiResult;

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
pub fn find_by_id(db: DB, id: ID) -> ApiResult<Option<Json<User>>> {
    rest::users::find_by_id(db.as_ref(), id)
        .map(|maybe_user| maybe_user.map(Json))
}

#[post("/", format = "application/json", data = "<input>")]
pub fn create(db: DB, input: Json<NewUser>) -> ApiResult<Created<Json<User>>> {
    rest::users::create(db.as_ref(), &input.into_inner())
        .map(|user| Created(format!("/members/{}", user.id), Some(Json(user))))
}
