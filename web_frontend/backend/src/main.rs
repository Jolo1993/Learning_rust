#![feature(proc_macro_hygiene, decl_macro)]

mod server;
mod routes;
mod models;

#[macro_use] extern crate rocket;

use crate::server::create_server;
use crate::routes::*;

fn main() {
    let server = create_server();
    server.mount("/", routes![index]);
    server.launch();
}