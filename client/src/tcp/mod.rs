pub mod utils {
    use std::io::{Read, Result, Write};
    use std::net::{Shutdown, TcpStream};

    use shared::Message;

    pub fn connect_to_server() -> Result<TcpStream> {
        return TcpStream::connect("localhost:7878");
    }

    pub fn disconnect_from_server(stream: &TcpStream) {
        stream.shutdown(Shutdown::Both).expect("Error while shutting down.");
    }

    pub fn send_message(message: &[u8], stream: &mut TcpStream) -> serde_json::error::Result<Message> {
        let message_size = message.len() as u32;
        stream.write(&message_size.to_be_bytes()).unwrap();
        stream.write_all(message).unwrap();

        let mut size_res = [0u8; 4];
        stream.read_exact(&mut size_res).unwrap();

        let size: u32 = u32::from_be_bytes(size_res);

        let mut data_res: Vec<u8> = vec![0u8; size.try_into().unwrap()];
        stream.read_exact(&mut data_res).unwrap();

        return serde_json::from_str::<Message>(&String::from_utf8_lossy(&data_res));
    }
}
