use super::schema::users;

use diesel::sql_types::{Int4};

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name="users"]
pub struct User {
    id: i32,
    email: String
}

