use std::net::TcpListener;

fn main() {
    establish_connection();
}

fn establish_connection() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established at 127.0.0.1:4000");
    }
}