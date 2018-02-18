extern crate rust_membership_api;
extern crate rocket;

fn main() {
    rust_membership_api::web::app::mount_app(rocket::ignite()).launch();
}
