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


pub fn run_rpc_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = lib::ThreadPool::new(10);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            rpc::handle_connection(stream);
        });
    }
}

