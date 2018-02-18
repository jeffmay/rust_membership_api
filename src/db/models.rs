use db::schema::users;

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name="users"]
pub struct User {
    id: i32,
    email: String
}
