use std::net::TcpStream;

fn main() {
    let hostname = "localhost";
    let port = 2222;

    match TcpStream::connect(format!("{}:{}", hostname, port)) {
        Ok(_) => println!("Port {} is listening.", port),
        Err(_) => println!("Port {} is not listening.", port),
    }
}
