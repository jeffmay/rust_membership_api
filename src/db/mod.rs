use core::ops::Deref;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request;
use rocket::request::{FromRequest, Request, State};
use core::borrow::Borrow;

pub mod models;
pub mod schema;

/// The Postgres connection type
pub type Connection = PgConnection;

/// The connection manager.
pub type ConnectionManager = r2d2_diesel::ConnectionManager<Connection>;

/// A pool of database connections.
pub type Pool = r2d2::Pool<ConnectionManager>;

/// Connection to the pool
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

/// A wrapper around an r2d2 pooled connection.
/// Returns a connection request guard type.
/// Called "DB" for short since it is used and passed along in every authenticated request.
pub struct DB {
    pub connection: PooledConnection,
}

impl DB {
    fn new(connection: PooledConnection) -> Self {
        DB { connection }
    }
}

/// Attempts to retrieve a single connection from the managed database pool.
/// If no pool is currently managed, fails with an `InternalServerError` status.
/// If no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DB::new(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl AsRef<Connection> for DB {
    fn as_ref(&self) -> &Connection {
        &self.connection
    }
}
