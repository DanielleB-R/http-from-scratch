use http::Method;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request;

static NOT_IMPLEMENTED: &str = "HTTP/1.1 501 Not Implemented\r\nConnection: close\r\n\r\n";
static HTTP_OK: &str = "HTTP/1.1 200 OK\r\n";

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
    let mut request_builder = match request::parse_request_line(&request_line) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error parsing request line");
            return;
        }
    };

    loop {
        let mut line = String::new();

        reader.read_line(&mut line).unwrap();
        if line == "\r\n" {
            break;
        }

        let mut pieces = line.splitn(2, ":");
        let name = pieces.next().unwrap();
        let value = pieces.next().unwrap().trim();
        request_builder = request_builder.header(name, value);
    }
    // Only handling empty requests for now;
    let request = request_builder.body(()).unwrap();
    println!("{:?}", request);

    if request.method() != Method::GET && request.method() != Method::HEAD {
        stream.write(NOT_IMPLEMENTED.as_bytes()).unwrap();
        return;
    }

    stream.write(HTTP_OK.as_bytes()).unwrap();
    stream.write(&[b'\r', b'\n']).unwrap();
    stream.write(request.uri().path().as_bytes()).unwrap();
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
