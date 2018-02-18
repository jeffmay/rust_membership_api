use r2d2;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DB(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DB, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DB(conn)),
            Err(_) => Outcome::Failure((status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DB {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
