use std::fs;
use std::io::{Read, Write};
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

/// Handles a HTTP request, and returns a response. Both the request and response are read and
/// written from/to the TCP streeam.
///
/// HTTP is a text-based protocol, and a request takes this format:
///
/// Method Request-URI HTTP-Version CRLF
/// headers CRLF
/// message-body
///
/// Responses have the following format:
///
/// HTTP-Version Status-Code Reason-Phrase CRLF
/// headers CRLF
/// message-body

fn handle_connection(mut stream: TcpStream) {
    // We are using 1024 here, because something shorter like 256 wouldn't be able to read the
    // entire request made from a browser, given the extra headers. We are not supporting requests
    // longer than 1024 bytes.

    let mut buffer = [0; 1024];

    // Read the tcp stream. If accessed from a browser or curl, this should set the following value
    // into our buf:

    // GET / HTTP/1.1
    // Host: localhost:7878
    // User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:78.0) Gecko/20100101 Firefox/78.0
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
    // Accept-Language: en-US,en;q=0.5
    // Accept-Encoding: gzip, deflate
    // Connection: keep-alive
    // Upgrade-Insecure-Requests: 1

    // or with `curl http://localhost:7878/`
    // GET / HTTP/1.1
    // Host: localhost:7878
    // User-Agent: curl/7.64.1
    // Accept: */*

    let bytes_read = stream
        // WARNING: If the request contains more than buf.len() bytes, then we won't end up reading
        // more than buf.len() bytes from the request, so the client will never get confirmation
        // that the request was read. So once `stream` is dropped, the connection will be forcefully
        // closed, resulting in a "connection reset" error in the client.
        .read(&mut buffer)
        .expect("unable to read request into buffer");

    // println!("bytes read:{}", bytes_read);
    // println!("request buffer:\n{}\n", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / ";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        // The write method on stream takes a &[u8] and sends those bytes directly down the connection.

        let bytes_written = stream
            .write(response.as_bytes())
            .expect("unable to write the response to the buffer.");

        println!("bytes written:{}", bytes_written);

        // Finally, flush will wait and prevent the program from continuing until all the bytes are
        // written to the connection; TcpStream contains an internal buffer to minimize calls to the
        // underlying operating system.

        stream
            .flush()
            .expect("unable to write all bytes from the internal buffer to the connection.");

        println!();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
