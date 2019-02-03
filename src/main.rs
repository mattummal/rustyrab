#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
extern crate lazy_static;
extern crate rocket_contrib;

mod blog;
mod context;
mod routes;
mod server;

use server::start_server;

fn main() {
    start_server().launch();
}
