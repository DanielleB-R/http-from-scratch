use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn respond(mut stream: TcpStream) {
    let mut input = vec![];
    stream.read_to_end(&mut input).unwrap();
    stream.write(&input).unwrap();
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
