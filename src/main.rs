use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request;

static NOT_IMPLEMENTED: &str = "HTTP/1.1 501 Not Implemented\r\nConnection: close\r\n\r\n";

fn respond(mut stream: TcpStream) {
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .expect("Can't clone stream for buffered reading"),
    );

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .expect("Problem reading request");
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
