// learn about tcp and http
// listen for tcp connections on a socket
// parse a small number of http reqs
// create a proper http response
// improve the throughout of our server with a thread pool

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
