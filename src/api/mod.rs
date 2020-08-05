mod endpoints;
use crate::db;
use std::fs;
use dotenv::dotenv;
extern crate dotenv;

#[derive(Debug, Deserialize)]
pub struct Config {
    url: String
}

pub fn run_rpc_server() {
    dotenv().ok();
    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = endpoints::mount(rocket);
    rocket.launch();
}