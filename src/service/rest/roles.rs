use db::Connection;
use db::models::Role;
use db::schema::*;
use diesel;
use diesel::prelude::*;
use service::rest::models::NewRole;
use service::rest::result::*;

pub fn create(c: &Connection, role: &NewRole) -> ApiResult<Role> {
    diesel::insert_into(roles::table)
        .values(role)
        .get_result(c)
        .into_api_result()
}
