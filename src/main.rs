use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9876").unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                println!("Broken Stream {}", e);
            }
            Ok(stream) => {
                handle_client(stream);
            } 
        }
    } 
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Err(e) => { println!("String Read Failed: {}", e); }
        Ok(size) => { println!("Message Size: {}", size); }
    }
    println!("{:?}", stream.peer_addr());
    println!("{:?}", buf);
}
