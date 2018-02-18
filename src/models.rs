
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    email: String,
    name: String,
}
