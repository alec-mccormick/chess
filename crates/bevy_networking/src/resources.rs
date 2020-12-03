use bytes::Bytes;
use crossbeam_channel::{Receiver, Sender};
use laminar::Socket;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Mutex
};

use super::{
    Message, WorkerInstructions,
    error::NetworkError,
    events::NetworkEvent,
    types::{Connection, SendConfig, SocketHandle}
};


pub struct NetworkResource {
    pub(crate) default_socket: Option<SocketHandle>,

    pub(crate) bound_sockets: Vec<SocketHandle>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) event_rx: Mutex<Receiver<NetworkEvent>>,
    pub(crate) message_tx: Mutex<Sender<Message>>,
    pub(crate) instruction_tx: Mutex<Sender<WorkerInstructions>>,
}

impl NetworkResource {
    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn connections_for_socket(&self, socket: SocketHandle) -> Vec<Connection> {
        self.connections
            .iter()
            .filter(|c| c.socket == socket)
            .cloned()
            .collect()
    }

    pub fn add_connection(&mut self, connection: Connection) {
        if self.has_connection(connection) {
            println!("Warning: attempted to add a connection that already exists");
            return;
        }

        self.connections.push(connection);
    }

    pub fn remove_connection(&mut self, connection: Connection) {
        let conn = self.connections.iter().position(|c| *c == connection);

        match conn {
            Some(idx) => {
                self.connections.remove(idx);
            }
            None => {
                println!("Warning: attempted to remove connection that doesn't exist");
            }
        }
    }

    pub fn has_connection(&self, connection: Connection) -> bool {
        self.connections.iter().any(|c| *c == connection)
    }

    pub fn bind<A: ToSocketAddrs>(&mut self, addr: A) -> Result<SocketHandle, NetworkError> {
        self.bind_with_transport(addr, Transport::Laminar(LaminarConfig::default()))
    }

    pub fn bind_with_transport<A: ToSocketAddrs>(
        &mut self,
        addr: A,
        transport: Transport,
    ) -> Result<SocketHandle, NetworkError> {
        match transport {
            Transport::Laminar(config) => self.bind_with_laminar(addr, config),
        }
    }

    fn bind_with_laminar<A: ToSocketAddrs>(
        &mut self,
        addr: A,
        config: LaminarConfig,
    ) -> Result<SocketHandle, NetworkError> {
        let cfg = config.into();

        let handle = SocketHandle::new();
        let socket = Socket::bind_with_config(addr, cfg)?;

        let instruction = WorkerInstructions::AddSocket(handle, socket);
        {
            let locked = self.instruction_tx.lock()?;
            locked.send(instruction)?;
        }

        self.bound_sockets.push(handle);

        if self.default_socket.is_none() {
            self.default_socket = Some(handle);
        }

        Ok(handle)
    }

    pub fn send(
        &self,
        addr: SocketAddr,
        message: &[u8],
        delivery: NetworkDelivery,
    ) -> Result<(), NetworkError> {
        self.send_with_config(addr, message, delivery, SendConfig::default())
    }

    pub fn broadcast(&self, message: &[u8], delivery: NetworkDelivery) -> Result<(), NetworkError> {
        self.broadcast_with_config(message, delivery, SendConfig::default())
    }

    pub fn send_with_config(
        &self,
        addr: SocketAddr,
        message: &[u8],
        delivery: NetworkDelivery,
        config: SendConfig,
    ) -> Result<(), NetworkError> {
        let socket = self.get_socket_or_default(config.socket)?;

        let msg = Message {
            destination: addr,
            delivery,
            socket_handle: socket,
            message: Bytes::copy_from_slice(message),
        };

        self.message_tx.lock()?.send(msg)?;

        Ok(())
    }

    pub fn broadcast_with_config(
        &self,
        message: &[u8],
        delivery: NetworkDelivery,
        config: SendConfig,
    ) -> Result<(), NetworkError> {
        let socket = self.get_socket_or_default(config.socket)?;

        let broadcast_to = self.connections_for_socket(socket);

        for conn in broadcast_to {
            let msg = Message {
                destination: conn.addr,
                delivery,
                socket_handle: socket,
                message: Bytes::copy_from_slice(message),
            };

            self.message_tx.lock()?.send(msg)?;
        }

        Ok(())
    }

    fn get_socket_or_default(
        &self,
        socket: Option<SocketHandle>,
    ) -> Result<SocketHandle, NetworkError> {
        let socket = socket
            .or(self.default_socket)
            .ok_or(NetworkError::NoDefaultSocket)?;

        match self.bound_sockets.contains(&socket) {
            true => Ok(socket),
            false => Err(NetworkError::NoSocket(socket)),
        }
    }
}

impl Drop for NetworkResource {
    fn drop(&mut self) {
        let locked = self.instruction_tx.lock().unwrap();
        locked.send(WorkerInstructions::Terminate).unwrap();
    }
}

