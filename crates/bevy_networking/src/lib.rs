use bevy::app::{AppBuilder, Events, EventReader, Plugin};
use bevy::ecs::prelude::*;


mod types;
mod error;
mod resources;
mod events;
mod worker;


pub use error::NetworkError;
pub use events::NetworkEvent;
pub use types::{Connection, SendConfig, SocketHandle, NetworkDelivery};
pub use resources::NetworkResource;


pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let network_resource = worker::start_worker_thread();

        app
            .add_event::<NetworkEvent>()
            .add_resource(network_resource)
            .add_system_to_stage(bevy::prelude::stage::EVENT,process_network_events.system());
    }
}



fn process_network_events(
    mut net: ResMut<NetworkResource>,
    mut network_events: ResMut<Events<NetworkEvent>>,
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
                _ => network_events.send(event),
            }
        }
    }

    for conn in added_connections {
        net.add_connection(conn);
        network_events.send(NetworkEvent::Connected(conn));
    }

    for conn in removed_connections {
        net.remove_connection(conn);
        network_events.send(NetworkEvent::Disconnected(conn));
    }
}