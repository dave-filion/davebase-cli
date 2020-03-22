#[macro_use] extern crate log;
use std::net::{TcpStream};
use std::io::{stdin, stdout, Read, Write, BufRead};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

static HOST_NAME: &str = "127.0.0.1:3333";

fn send_good_get() {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("connected to port 3333");
            let msg = b"GET dave::id";
            // recv buffer limit is 128 bytes
            stream.write(msg).expect("Couldnt write to stream");
            println!("Sent message");

            // read back result
            let mut buff = [0 as u8; 128];
            let size = stream.read(&mut buff).unwrap();
            println!("Read {} bytes: {}", size, from_utf8(&buff[0..size]).unwrap());
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send_good_msg () {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("connected to port 3333");
            let msg = b"SET dave::id 12341223777";
            // recv buffer limit is 128 bytes
            stream.write(msg).expect("Couldnt write to stream");

           println!("Sent message")
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn send_bad_msg () {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {

            println!("Writing bad message");
            let bad_msg = b"thiswontworkdave::id 12341223777";
            stream.write(bad_msg).expect("Couldnt write to stream");

            println!("Sent message")
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn main() {
    let mut buff = String::new();

    // look until kill command received
    loop {
        print!("davebase-cli:> ");
        stdout().flush();

        // wait for user input
        let read_size = stdin().lock().read_line(&mut buff).unwrap();

        // remove newlines
        trim_newline(&mut buff);

        println!("User inputted: {}", buff);

        // if command was quit, exit
        match buff.to_lowercase().as_str() {
            "quit" | "q" | "exit"  => {
                break;
                println!("Exiting...");
            },
            _ => buff.clear(),
        }

    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
