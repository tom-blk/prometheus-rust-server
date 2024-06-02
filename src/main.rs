use std::{
    fs,
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread,
};

use prometheus_rust::ThreadPool;

//#todo
// Proper Eroor handling on all of the unwraps
// jwt auth
// different routes
// database connection
// logging

fn main() {
    let result = TcpListener::bind("127.0.0.1:7878");

    match result {
        Ok(listener) => {
            println!("Server started on 127.0.0.1:7878");
    
            for stream in listener.incoming(){
                let stream = stream.unwrap();

                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
        },
        Err(error) => println!("Failed to start server: {}", error),
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
            .lines()
            .next()
            .unwrap()
            .unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")  
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

