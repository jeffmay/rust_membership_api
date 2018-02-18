extern crate rust_membership_api;
extern crate rocket;

use rust_membership_api::config::AppConfig;

fn main() {
    let config = AppConfig::from_env();
    rust_membership_api::web::app::mount_app(rocket::ignite(), config).launch();
}
