use std::fs;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

fn main() {
    // Simple listener binding to a port
    let listener = TcpListener::bind("127.0.0.1:4004").unwrap();
    // TcpListener returns an interator of connections.
    for conn in listener.incoming() {
        let stream = conn.unwrap();
        println!("Got a connection");

        handle_connection(stream);
    }
}

/// Handles the connection of the tcp stream
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Printing the reuqest:
    // println!("Request {}", String::from_utf8_lossy(&buffer[..]));

    // Get the contents of the file
    let file_content = fs::read_to_string("pages/hello.html").unwrap_or(String::from(""));

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        file_content.len(),
        file_content,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

