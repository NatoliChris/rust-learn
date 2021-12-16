use std::fs;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::time::Duration;

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

    // Let's pull out the requests
    let buffcontents = buffer.splitn(3, |ch| *ch == 32 as u8).collect::<Vec<&[u8]>>();

    // Simple if else to match the filename
    // NOTE: maybe I can change this to a match?
    let (status_line, filename) = if buffcontents[1] == b"/sleep" {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffcontents[1] == b"/" {
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

