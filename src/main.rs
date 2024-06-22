use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use server::{
    request::{Request, RequestError},
    response::Response,
    HttpStatus,
};

#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to convert bytes to string: {0}")]
    ParseError(#[from] std::str::Utf8Error),
    #[error("Request error: {0}")]
    RequestError(#[from] RequestError),
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let http_string = std::str::from_utf8(&buffer)?.trim_end_matches("\0");
    let request: Request = http_string.try_into()?;
    let response;
    match request.endpoint().as_str() {
        "/" => response = Response::new(HttpStatus::Ok),
        _ => response = Response::new(HttpStatus::NotFound),
    }
    stream.write(response.http_string().as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_connection(_stream) {
                    println!("{}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
