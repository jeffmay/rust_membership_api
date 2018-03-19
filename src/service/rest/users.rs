use db::{Connection, DB};
use db::models::{ID, User};
use db::schema::*;
use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use service::rest::models::NewUser;
use service::rest::result::{ApiError, ApiResult, IntoApiResult};

pub fn create(c: &Connection, guest: NewUser) -> ApiResult<User> {
    diesel::insert_into(users::table)
        .values(&guest)
        .get_result(c)
        .into_api_result()
}

pub fn find_by_id(c: &Connection, id: ID) -> ApiResult<Option<User>> {
    println!("Searching for user with id={}", id);
    users::table.find(id).first::<User>(c).into_api_result()
}
