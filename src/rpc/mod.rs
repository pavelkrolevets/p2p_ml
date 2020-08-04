
mod rpc;
mod lib;

use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use serde_json::{Deserializer, Value};
use std::borrow::Borrow;
use std::borrow::Cow;
use serde::Deserialize;

use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    url: String
}

pub fn run_rpc_server() {

    let conf_file = fs::File::open("config.json").expect("file should open read only");
    let conf : serde_json::Value = serde_json::from_reader(conf_file).expect("file should be proper JSON");
    let url = conf.get("url").unwrap().as_str().unwrap();
    println!("Running listener at {:?}", url);

    let listener = TcpListener::bind(url).unwrap();
    let pool = lib::ThreadPool::new(10);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            rpc::handle_connection(stream);
        });
    }
}

