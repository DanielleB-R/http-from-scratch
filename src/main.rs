use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request;

static NOT_IMPLEMENTED: &str = "HTTP/1.1 501 Not Implemented\r\nConnection: close\r\n\r\n";

fn respond(mut stream: TcpStream) {
    let mut input = String::new();
    stream.read_to_string(&mut input).unwrap();
    let request_line = input.lines().next().unwrap();
    let request = match request::parse_request_line(&request_line) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error parsing request line");
            return;
        }
    };
    println!("{:?}", request);
    if request.method != "GET" && request.method != "HEAD" {
        stream.write(NOT_IMPLEMENTED.as_bytes()).unwrap();
        return;
    }

    stream.write(request.url.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind to port 8888");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(move || respond(stream));
            }
            Err(e) => eprintln!("Error establishing connection {}", e),
        }
    }
}
