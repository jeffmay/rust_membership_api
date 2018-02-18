#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use rocket::{Request, State, Outcome};

use rocket::request::{self, FromRequest};
use rocket::http::Status;
use r2d2_diesel::ConnectionManager;
use rocket::outcome::Outcome::*;

use std::ops::Deref;

pub mod models;
pub mod schema;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod web;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
