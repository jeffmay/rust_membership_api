use db::schema::*;
use diesel::insertable::Insertable;

/// A model used to create a new user in the database.
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    email_address: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}