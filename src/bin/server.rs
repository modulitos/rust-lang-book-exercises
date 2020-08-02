use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    // bind to our localhost, at port 7878 (which is "rust" when typed into a phone)

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 256];

    // Read the tcp stream. If accessed from a browser or curl, this should set the following value
    // into buf:

    // GET / HTTP/1.1
    // Host: localhost:7878
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:78.0) Gecko/20100101 Firefox/78.0
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
    // Accept-Language: en-US,en;q=0.5

    // or with `curl http://localhost:7878/hey`
    // GET /hey HTTP/1.1
    // Host: localhost:7878
    // User-Agent: curl/7.64.1
    // Accept: */*

    stream
        .read(&mut buf)
        .expect("unable to read request into buffer");

    println!("buf:\n{}", String::from_utf8_lossy(&buf));
}
