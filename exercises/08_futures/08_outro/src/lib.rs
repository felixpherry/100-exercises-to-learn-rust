// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use request::{Request, RequestError};

pub mod data;
pub mod request;
pub mod store;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Socket error: {0}")]
    SocketError(#[from] std::io::Error),
    #[error("Error parsing buffer to string: {0}")]
    ParseError(#[from] std::str::Utf8Error),
    #[error("Request error: {0}")]
    RequestError(#[from] RequestError),
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let request: Request = std::str::from_utf8(&buffer)?
        .trim_end_matches("\0")
        .try_into()?;
    println!("request: {:?}", request);

    stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
    stream.flush()?;
    Ok(())
}

pub fn launch() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let address = listener.local_addr()?;
    println!("Listening on: {}", address);

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}
