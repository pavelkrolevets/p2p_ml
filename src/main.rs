#![feature(plugin)]
#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;


mod  rpc;
mod  api;
mod  db;

use std::fmt::Error;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub fn parse_stdin(args: Vec<&str>) -> Result<(), Error>{

    match args[0] {
        "run" => api::run_rpc_server(),
        _ => println!("Please input a correct command.")
    };
    Ok(())
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    loop {
        stdout.write_all(b"input: ").unwrap();
        stdout.flush().unwrap();

        let input = stdin.read_line();

        if let Ok(Some(input)) = input {
            let args: Vec<&str> = input.as_str().split_whitespace().collect();
            if args.len() == 0 {
                stdout.write_all("Please input something".as_bytes()).unwrap();
                stdout.write_all(b"\n").unwrap();
            } else {
                if let Err(e) = parse_stdin(args){
                    stdout.write_all(Error.to_string().as_bytes());
                    stdout.write_all(b"\n").unwrap();
                }
                sleep(Duration::from_millis(100));
            }
        } else {
            stdout.write_all(b"Error\n").unwrap();
        }
    }
}

