use bytes::Bytes;
use laminar::Socket;
use std::{
    fmt,
    net::{SocketAddr, ToSocketAddrs}
};
use uuid::Uuid;




#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SocketHandle(uuid::Uuid);

impl SocketHandle {
    pub(crate) fn new() -> Self {
        // We're using UUID here to mirror the way bevy currently treats asset handles. Since sockets handles are specific to a single process, and it's
        // unlikely anyone will have a large number of sockets, we could switching to a u32.
        Self(Uuid::new_v4())
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Connection {
    pub addr: SocketAddr,
    pub socket: SocketHandle,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.addr)
    }
}



#[derive(Default)]
pub struct SendConfig {
    pub socket: Option<SocketHandle>, // if none, use the default socket
}

#[derive(Debug)]
pub(crate) struct Message {
    pub(crate) message: Bytes,
    pub(crate) delivery: NetworkDelivery,
    pub(crate) socket_handle: SocketHandle,
    pub(crate) destination: SocketAddr,
}

pub(crate) enum WorkerInstructions {
    AddSocket(SocketHandle, Socket),
    Terminate,
}