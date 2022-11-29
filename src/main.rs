use std::io::Write;
// Uncomment this block to pass the first stage
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //

    let ping_response = b"+PONG\r\n";

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted Connection!");
                stream.write(ping_response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
