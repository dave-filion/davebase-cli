#[macro_use] extern crate log;
use std::net::{TcpStream, Shutdown};
use std::io::{stdin, stdout, Read, Write, BufRead, ErrorKind};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

static HOST_NAME: &str = "127.0.0.1:3333";

// Sends message to DB, reads back response
fn send_to_db(msg: &str) -> std::io::Result<String> {
    if let Ok(mut stream) = TcpStream::connect(HOST_NAME) {
        // recv buffer limit is 128 bytes
        stream.write(msg.as_bytes())
            .expect("Couldnt write to stream");

        // read back result
        let mut buff = [0 as u8; 128];
        let size = stream.read(&mut buff).unwrap();
        let response = from_utf8(&buff[0..size]).unwrap();

        // shutdown read
        stream.shutdown(Shutdown::Both);

        Ok(response.to_string())
    } else {
        Err(std::io::Error::new(ErrorKind::Other, "Couldnt connect to host"))
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
        let msg_str = buff.as_str();

        if let "QUIT" | "EXIT" = msg_str.to_uppercase().as_str() {
            println!("Exiting...");
            break;
        }

        match send_to_db(buff.as_str()) {
            Ok(result) => {
                println!("davebase-cli:> {}", result);
            },
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }

        buff.clear();

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


// test require database to be running
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let set_msg = "SET test_val_1 thisvalue";
        let result = send_to_db(set_msg);
        // expect OK response on successful get
        assert_eq!(result.unwrap(), "OK");

        // wait a sec
        sleep(Duration::from_secs(1));

        let get_msg = "GET test_val_1";
        let result = send_to_db(get_msg);
        assert_eq!(result.unwrap(), "thisvalue")
    }

}