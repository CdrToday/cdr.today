use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream).unwrap();
    }
}

fn handle_connection(stream: Result<TcpStream, Error>) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream?.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    Ok(())
}
