use diesel::*;
use db::schema::*;
use db::models::ID;

/// A model used to create a new [User] in the database.
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    email_address: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

/// A model used to create a new [Role] in the database.
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name="roles"]
pub struct NewRole {
    committee_id: Option<ID>,
    role: String
}
