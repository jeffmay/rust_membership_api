#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod web;
