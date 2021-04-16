use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind to port 8888");

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                stream.write(&[b'a']).unwrap();
            }
            Err(e) => eprintln!("Error establishing connection {}", e),
        }
    }
}
