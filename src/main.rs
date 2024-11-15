#![allow(unused_imports)]
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("accepted new connection");
    let mut buf = [0; 4096];
    loop {
        let read_count = stream.read(&mut buf)?;
        if read_count == 0 {
            break;
        }
        stream.write(b"+PONG\r\n").unwrap();
    }
    Ok(())
}
