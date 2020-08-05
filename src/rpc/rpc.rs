use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;
use std::net::TcpStream;
use serde_json::{Deserializer, Value};
use std::borrow::Borrow;
use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    res: String
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";

    //
    // let (status_line, result) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else if  buffer.starts_with(post) {
    //     let str = String::from_utf8_lossy(&buffer[..]).to_string();
    //     for i in str.split("\r\n") {
    //         println!("Request: {}", &i);
    //     }
    //     let resp = Response {
    //                 res: "Some result".to_string()
    //             };
    //     let result = serde_json::to_string(&resp).unwrap();
    //     ("HTTP/1.1 200 OK\r\n\r\n", result.as_str().deref())
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };

    let (status_line, r) = if buffer.starts_with(post) {
        let str = String::from_utf8_lossy(&buffer[..]).to_string();
        for i in str.split("\r\n") {
            println!("Request: {}", &i);
        }
        let resp = Response {
            res: "Some result".to_string()
        };
        let result = serde_json::to_string(&resp).unwrap();
        ("HTTP/1.1 200 OK\r\n\r\n", result)
    } else {
        let resp = Response{
            res: "Error".to_string()
        };
        let result = serde_json::to_string(&resp).unwrap();
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", result)
    };

    // let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, r);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}