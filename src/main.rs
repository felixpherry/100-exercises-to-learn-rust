use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use server::{response::Response, HttpStatus};

#[derive(Debug, thiserror::Error)]
enum ServerError {
    #[error("Error writing buffer: {0}")]
    WriteError(#[from] std::io::Error),
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ServerError> {
    let response = Response::new(HttpStatus::Ok);
    println!("response: {:?}", response.http_string());
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
