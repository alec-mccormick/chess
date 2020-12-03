use bytes::Bytes;

use super::{
    error::NetworkError,
    types::Connection,
};


// #[derive(Debug)]
// pub struct ClientConnected(Connection);
//
// #[derive(Debug)]
// pub struct ClientDisconnected(Connection);
//
// #[derive(Debug)]
// pub struct MessageReceived(Connection, Bytes);
//
// #[derive(Debug)]
// pub struct SendMessageError(NetworkError);


#[derive(Debug)]
pub enum NetworkEvent {
    Connected(Connection),
    Disconnected(Connection),
    Message(Connection, Bytes),
    SendError(NetworkError),
}