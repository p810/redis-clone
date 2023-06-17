use std::net::TcpListener;

fn main() {
    println!("Starting the faux redis server");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error accepting a new connection: {}", e);
            }
        }
    }
}
