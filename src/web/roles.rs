use db::DB;
use db::models::Role;
use rocket::response::status::Created;
use rocket::Route;
use rocket_contrib::Json;
use service::rest::roles;
use service::rest::models::NewRole;
use service::rest::result::ApiResult;

/// The list of all routes in this module.
/// Be sure to add new routes to this list or they will not be registered in the router.
pub fn routes() -> Vec<Route> {
    routes![
        create,
    ]
}

#[post("/", format = "application/json", data = "<input>")]
pub fn create(db: DB, input: Json<NewRole>) -> ApiResult<Created<Json<Role>>> {
    roles::create(db.as_ref(), &input.into_inner())
        .map(|role| Created(format!("/roles/{}", role.id), Some(Json(role))))
}
