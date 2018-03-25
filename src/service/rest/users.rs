use db::Connection;
use db::models::{ID, User};
use db::schema::*;
use diesel;
use diesel::prelude::*;
use service::rest::models::NewUser;
use service::rest::result::*;

/// Create the given [NewUser] and return the fully constructed [User].
pub fn create(c: &Connection, guest: &NewUser) -> ApiResult<User> {
    diesel::insert_into(users::table)
        .values(guest)
        .get_result(c)
        .into_api_result()
}

pub fn find_by_id(c: &Connection, id: ID) -> ApiResult<Option<User>> {
    users::table.find(id).first::<User>(c).into_api_result()
}
