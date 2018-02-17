extern crate rocket;
extern crate rust_membership_api;

fn main() {
    rust_membership_api::web::app::mount_app(rocket::ignite()).launch();
}
