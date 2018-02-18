#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod config;
pub mod db;
pub mod models;
pub mod schema;
pub mod web;
