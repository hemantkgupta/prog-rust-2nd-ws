use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::thread;

pub fn networking0_work(){
    thread::spawn(start_server);
    start_client();
}

fn start_client(){
    let player_stream = TcpStream::connect("127.0.0.1:8000").expect("Couldn't connect");

    let mut writer = BufWriter::new(&player_stream);
    // Write bytes to stream
    writer.write_all("Hello\n".as_bytes()).expect("Could not write");
    writer.flush().expect("could not flush"); 

    let mut reader = BufReader::new(&player_stream);
    let mut response = String::new();

    // Read one line from stream into a string
    reader.read_line(&mut response).expect("Could not read");
    println!("Client received: {}", response);

}



fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    println!("Client connected");

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    // Read one line from stream into a string
    reader.read_line(&mut response).expect("could not read");
    println!("Server received: {}", response);

    let mut writer = BufWriter::new(&stream);
    // Write bytes to stream
    writer.write_all("Hi\n".as_bytes()).expect("could not write");
    writer.flush().expect("could not flush"); 

       
}