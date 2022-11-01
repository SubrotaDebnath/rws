use std::net::{TcpListener, TcpStream};
use  std::io::prelude::* ;

fn main() {
    let  listener =  TcpListener::bind("127.0.0.1:7878").expect("Connection Field");

    for stream in listener.incoming()   {
        let  stream = stream.unwrap();
        println!("Connected to server");
        handel_connection(stream);

    }
}


fn handel_connection(mut stream:TcpStream){
     let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response ="HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}