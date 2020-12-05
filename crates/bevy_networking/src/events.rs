use bytes::Bytes;
use super::{error::NetworkError, types::Connection};


#[derive(Debug)]
pub struct ClientConnected(pub Connection);

#[derive(Debug)]
pub struct ClientDisconnected(pub Connection);

#[derive(Debug)]
pub struct MessageReceived(pub Connection, pub Bytes);

#[derive(Debug)]
pub struct SendError(pub NetworkError);


#[derive(Debug)]
pub enum NetworkEvent {
    Connected(Connection),
    Disconnected(Connection),
    Message(Connection, Bytes),
    SendError(NetworkError),
}