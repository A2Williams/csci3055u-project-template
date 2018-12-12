use std::io::prelude::*;
use std::fs;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {

    // establish a connection to local computer on port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // create a buffer for handling requests
    let mut buffer = [0; 512];

    // read the HTTP request
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // response to the HTTP request with our HTML
    let content = fs::read_to_string("rust.html").unwrap();

    let resp = format!("HTTP/1.1 200 OK \r\n\r\n{}", content);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}