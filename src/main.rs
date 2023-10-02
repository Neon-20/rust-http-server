use std::fs;
// Listen to Tcp request
use std::net::TcpListener; //Listen to TcpListener
use std::net::TcpStream; //Reading data from Tcp stream
use std::io::prelude::*;


fn main() {
    let listener = 
    TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
    let stream = stream.unwrap();
    handleConnection(stream);
    }
}

//reading data from the tcpstream and print it out
fn handleConnection(mut stream:TcpStream){
    // Creating a buffer to hold the data
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get){
        let contents = fs::read_to_string("index.html").unwrap();
    
        let response = format! 
        (
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        
        let response = format! 
        (
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    
}

// Response format
// ex: HTTP/1.1 200 OK\r\n\r\n