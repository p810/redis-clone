use std::io::{Read, Write, ErrorKind};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Starting the faux redis server");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                recv_incoming(_stream);
            }
            Err(e) => {
                println!("error accepting a new connection: {}", e);
            }
        }
    }
}

fn recv_incoming(mut client: TcpStream) {
    println!("Creating buffer for client's incoming bytes");
    
    client.set_nonblocking(true).expect("Failed to set nonblocking to true on the client socket");

    client.set_read_timeout(Some(Duration::from_millis(5000))).expect("Failed to set read_timeout on the client socket");

    let mut buffer = vec![0; 128];

    loop {
        match client.read(&mut buffer) {
            Ok(bytes) if bytes == 0 => {
                println!("Received Ok(0) from read call");
                break;
            },
            Ok(_) =>
                respond_to_ping(&mut buffer, &mut client),
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                println!("Sleeping for two seconds to wait for socket to become available");
                thread::sleep(Duration::from_millis(2000));
            },
            Err(e) => {
                let ref kind = e.kind();

                match kind {
                    ErrorKind::UnexpectedEof | ErrorKind::ConnectionAborted =>
                        println!("Client disconnected ({:?})", kind),
                    _ =>
                        panic!("Encountered an error case, {}", e),
                };

                break;
            },
        };
    }

    println!("Exiting recv_incoming");
}

fn respond_to_ping(buf: &mut Vec<u8>, client: &mut TcpStream) {
    if buf.len() == 0 {
        return;
    }

    println!("Read {} bytes from the client's socket into a buffer", buf.len());

    match client.write("+PONG\r\n".as_bytes()) {
        Ok(bytes) => println!("Wrote {} bytes to the client socket", bytes),
        Err(e) => panic!("Failed to write some or all data to the other end socket, {:?}", e),
    };

    buf.clear();
}
