use laminar::{Socket, Packet};
use std::thread;
use std::net::SocketAddr;
use std::str::FromStr;

fn main() {
    println!("Sender Main");

    // Creates the socket
    let mut socket = Socket::bind("127.0.0.1:12345").unwrap();
    let packet_sender = socket.get_packet_sender();
    // Starts the socket, which will start a poll mechanism to receive and send messages.
    let _thread = thread::spawn(move || {
        println!("START POLLING CALLED");
        socket.start_polling();
    });

    // Bytes to sent
    let destination = SocketAddr::from_str("127.0.0.1:12346").unwrap();
    let bytes = b"Hello there!".to_vec();


    let reliable_ordered = Packet::reliable_ordered(destination, bytes, Some(1));

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("After sleep");

    // Sends the created packets
    packet_sender.send(reliable_ordered).unwrap();
    println!("Packet Sent!");

    std::thread::sleep(std::time::Duration::from_secs(2));
}