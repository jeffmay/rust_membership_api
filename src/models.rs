
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Member{
    id: u32,
    email: String,
    name: String,
}
