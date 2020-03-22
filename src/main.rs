use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

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
    println!("Starting...");
    send_good_msg();

    sleep(Duration::from_secs(5));

    send_bad_msg();

    sleep(Duration::from_secs(5));

    send_good_get();
}
