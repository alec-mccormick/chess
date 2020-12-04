use bytes::Bytes;
use laminar::{Config, Socket};
use std::{
    fmt,
    net::{SocketAddr, ToSocketAddrs},
    time::Duration
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NetworkDelivery {
    UnreliableUnordered,
    UnreliableSequenced(Option<u8>),
    ReliableUnordered,
    ReliableSequenced(Option<u8>),
    ReliableOrdered(Option<u8>),
}







pub enum Transport {
    Laminar(LaminarConfig),
}

pub struct LaminarConfig {
    pub idle_connection_timeout: Duration,
    pub heartbeat_interval: Option<Duration>,
    pub max_packets_in_flight: u16,
}

impl Default for LaminarConfig {
    fn default() -> Self {
        LaminarConfig {
            idle_connection_timeout: Duration::from_millis(5000),
            heartbeat_interval: Some(Duration::from_millis(1000)),
            max_packets_in_flight: 1024,
        }
    }
}

impl From<LaminarConfig> for Config {
    fn from(cfg: LaminarConfig) -> Self {
        Config {
            idle_connection_timeout: cfg.idle_connection_timeout,
            heartbeat_interval: cfg.heartbeat_interval,
            max_packets_in_flight: cfg.max_packets_in_flight,
            ..Default::default()
        }
    }
}
