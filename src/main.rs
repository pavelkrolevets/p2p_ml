#[macro_use]
extern crate termion;

mod  rpc;

use std::fmt::Error;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub fn parse_stdin(args: Vec<&str>) -> Result<(), Error>{

    match args[0] {
        "run" => rpc::run_rpc_server(),
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

