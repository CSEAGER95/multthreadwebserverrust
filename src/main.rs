use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:7878"; //this is an ip addr that represents my computer, the port is a random port that usually doesn't accept HTTP requests.
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection established");
    }
}
