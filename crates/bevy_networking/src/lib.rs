use bevy::app::{AppBuilder, Events, EventReader, Plugin};
use bevy::ecs::prelude::*;
use bevy::prelude::stage;

mod types;
mod error;
mod resources;
pub mod events;
mod worker;


use events::NetworkEvent;
pub use error::NetworkError;
pub use types::{Connection, SendConfig, SocketHandle, NetworkDelivery};
pub use resources::NetworkResource;
pub use bytes::Bytes;


pub struct NetworkingPlugin;
impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let network_resource = worker::start_worker_thread();

        app
            .add_event::<events::ClientConnected>()
            .add_event::<events::ClientDisconnected>()
            .add_event::<events::MessageReceived>()
            .add_event::<events::SendError>()
            .add_resource(network_resource)
            .add_system_to_stage(stage::EVENT, process_network_events.system());
    }
}



fn process_network_events(
    mut net: ResMut<NetworkResource>,
    mut connected_events: ResMut<Events<events::ClientConnected>>,
    mut disconnected_events: ResMut<Events<events::ClientDisconnected>>,
    mut message_events: ResMut<Events<events::MessageReceived>>,
    mut error_events: ResMut<Events<events::SendError>>,
) {
    let mut added_connections: Vec<Connection> = Vec::new();
    let mut removed_connections: Vec<Connection> = Vec::new();

    {
        let locked = match net.event_rx.lock() {
            Ok(l) => l,
            // this system is the only consumer of `event_rx`, so if this lock is poisoned that means
            // a previous iteration of our thread panic'd without taking down the game. We'll
            // bravely try and soldier on and continue to process network event's, but it's pretty
            // bad.
            Err(p) => p.into_inner(),
        };

        while let Ok(event) = locked.try_recv() {
            match event {
                NetworkEvent::Connected(conn) => {
                    if !net.has_connection(conn) && !added_connections.contains(&conn) {
                        added_connections.push(conn);
                    }
                }
                NetworkEvent::Disconnected(conn) => {
                    if net.has_connection(conn) && !removed_connections.contains(&conn) {
                        removed_connections.push(conn);
                    }
                }
                NetworkEvent::Message(conn, bytes) => {
                    message_events.send(events::MessageReceived(conn, bytes));
                }
                NetworkEvent::SendError(error) => {
                    error_events.send(events::SendError(error));
                }
            }
        }
    }

    for conn in added_connections {
        net.add_connection(conn);
        connected_events.send(events::ClientConnected(conn));
    }

    for conn in removed_connections {
        net.remove_connection(conn);
        disconnected_events.send(events::ClientDisconnected(conn));
    }
}