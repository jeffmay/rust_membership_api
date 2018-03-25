use db::schema::*;

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

#[derive(Debug, Serialize, Insertable, Queryable)]
#[table_name="roles"]
pub struct Role {
    pub id: ID,

    /// The committee for which this role is limited.
    /// None implies that the role is chapter-wide
    pub committee_id: Option<ID>,

    pub user_id: ID,

    /// The name of the role. Must be unique per committee.
    pub role: String,
}

pub enum ElectionStatus {
    Draft,
    Published,
    Final,
}

