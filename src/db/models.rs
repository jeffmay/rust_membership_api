use db::schema::*;
use diesel::*;
use diesel::deserialize::{self, FromSql};
use diesel::insertable::Insertable;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use std::str;

/// The standard ID type for the database
pub type ID = i32;

/// A user from the `users` table. Can either be a guest or a member.
#[derive(Debug, Serialize, Insertable, Queryable)]
#[table_name="users"]
pub struct User {
    pub id: ID,
    pub email_address: Option<String>,
    pub normalized_email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub biography: Option<String>,
}

impl User {
    pub fn name(self) -> Option<String> {
        let full_name = format!(
            "{} {}",
            self.first_name.unwrap_or("".to_string()),
            self.last_name.unwrap_or("".to_string())
        ).trim().to_string();
        if full_name.is_empty() {
            None
        } else {
            Some(full_name)
        }
    }
}

pub enum ElectionStatus {
    Draft,
    Published,
    Final,
}

