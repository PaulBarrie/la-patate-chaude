use std::net::TcpStream;

use shared::Message;

use crate::tcp::utils;
use crate::tcp::utils::{connect_to_server, disconnect_from_server};

pub fn run() {
    let stream = connect_to_server();
    match stream {
        Ok(mut stream) => {
            println!("Successfully connected to server !");
            send_message(&Message::Hello, &mut stream);
            disconnect_from_server(&stream);
        }
        Err(e) => {
            println!("Cannot connect to server: {}", e);
        }
    }
}


fn send_message(message: &Message, mut stream: &mut TcpStream) {
    println!("Sending message: {:?}\n", message);
    let response = utils::send_message(
        serde_json::to_string(message).unwrap().as_bytes(),
        &mut stream);
    println!("Received {:?}\n", response);
}
