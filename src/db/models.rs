use diesel::*;
use db::schema::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use std::str;

#[derive(Debug, Deserialize, Serialize, Insertable, Queryable)]
#[table_name="users"]
pub struct User {
    id: i32,
    email_address: Option<String>,
    normalized_email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    biography: Option<String>,
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

