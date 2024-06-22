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
    #[error("No user agent header")]
    UserAgentError,
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let http_string = std::str::from_utf8(&buffer)?.trim_end_matches("\0");
    let request: Request = http_string.try_into()?;
    let mut response;
    match request.endpoint().as_str() {
        "/" => response = Response::new(HttpStatus::Ok, None),
        "/user-agent" => {
            let response_body = request
                .headers()
                .get("user-agent")
                .ok_or(ServerError::UserAgentError)?
                .to_owned();

            response = Response::new(HttpStatus::Ok, Some(response_body.clone()));
            response.set_headers("Content-Type".to_string(), "text/plain".to_owned());
            response.set_headers(
                "Content-Length".to_string(),
                response_body.len().to_string(),
            );
        }
        other => {
            if other.starts_with("/echo/") {
                let response_body = other.replace("/echo/", "");
                response = Response::new(HttpStatus::Ok, Some(response_body.clone()));
                response.set_headers("Content-Type".to_string(), "text/plain".to_owned());
                response.set_headers(
                    "Content-Length".to_string(),
                    response_body.len().to_string(),
                );
            } else {
                response = Response::new(HttpStatus::NotFound, None)
            }
        }
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
