use rust_lang_book::thread_pool::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};

/// Building a Multi-Threaded Web Server. Final project for the Rust Lang book:
/// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

fn main() {
    // bind to our localhost, at port 7878 (which is "rust" when typed into a phone)

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Create a thread pool that executes connections asynchronously. There are never more than four
    // threads created, so our system won’t get overloaded if the server receives a lot of requests.
    // If we make a request to /sleep, the server will be able to serve other requests by having
    // another thread run them.

    // Using a thread pool is just one of many ways to improve the throughput of a web server.
    // Other options are the fork/join model and the single-threaded async I/O model.

    let pool = ThreadPool::new(4);

    // To simulate the server shutting down gracefully, we can call `incoming().take(2)` to make it
    // shutdown after 2 requests.

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
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

    let _bytes_read = stream
        // WARNING: If the request contains more than buf.len() bytes, then we won't end up reading
        // more than buf.len() bytes from the request, so the client will never get confirmation
        // that the request was read. So once `stream` is dropped, the connection will be forcefully
        // closed, resulting in a "connection reset" error in the client.
        .read(&mut buffer)
        .expect("unable to read request into buffer");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("{}", filename)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // The write method on stream takes a &[u8] and sends those bytes directly down the connection.

    let _bytes_written = stream
        .write(response.as_bytes())
        .expect("unable to write the response to the buffer.");

    // Finally, flush will wait and prevent the program from continuing until all the bytes are
    // written to the connection; TcpStream contains an internal buffer to minimize calls to the
    // underlying operating system.

    stream
        .flush()
        .expect("unable to write all bytes from the internal buffer to the connection.");
}
