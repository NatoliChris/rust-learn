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

    // A simple get request starting string
    let get_start_string = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_start_string) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Read the file contents
    let file_content = fs::read_to_string(format!("pages/{}", filename)).unwrap_or(String::from(""));

    // Format the response
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        file_content.len(),
        file_content,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

