use laminar::{Socket, SocketEvent};
use std::thread;

fn main() {
    // Creates the socket
    let mut socket = Socket::bind("127.0.0.1:12346").unwrap();
    let event_receiver = socket.get_event_receiver();

    // Starts the socket, which will start a poll mechanism to receive and send messages.
    let _thread = thread::spawn(move || socket.start_polling());

    // Waits until a socket event occurs
    let result = event_receiver.recv();

    match result {
        Ok(socket_event) => {
            match  socket_event {
                SocketEvent::Packet(packet) => {
                    let received_data: &[u8] = packet.payload();
                    println!("RECEIVED PACKET: {:?}", String::from_utf8_lossy(&*received_data));
                },
                SocketEvent::Connect(connect_event) => {
                    println!("CLIENT CONNECTED");
                },
                SocketEvent::Timeout(timeout_event) => {
                    println!("CLIENT TIMEOUT");
                },
                _ => {
                    println!("UNKNOWN EVENT");
                }
            }
        }
        Err(e) => {
            println!("Something went wrong when receiving, error: {:?}", e);
        }
    }
}