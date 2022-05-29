use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7979").unwrap();
            listener.incoming().for_each(|stream: Result<TcpStream, std::io::Error>| {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream);
    });
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


    let mut files: HashMap<&str, &str> = HashMap::new();
    files.insert("/index.html", "html/index.html");
    
    let request: std::borrow::Cow<str> = String::from_utf8_lossy(&buffer[..]);
    
    let mut file_name: &str = request.split_whitespace().nth(1).unwrap();

	if !files.contains_key(file_name) {
		file_name = "html/index.html";
	}
	if file_name.starts_with('/') {
		file_name = &file_name[1..];
	}



    let mut file: File = File::open(file_name).unwrap();
    let mut contents: String = String::new();
    file.read_to_string(&mut contents);
    let response: String = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
    

}