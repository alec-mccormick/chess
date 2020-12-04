use bevy::app::{ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy::prelude::*;

use std::net::SocketAddr;
use std::time::Duration;

use bevy_networking::{
    NetworkDelivery, NetworkEvent, NetworkResource, NetworkingPlugin,
};

const SERVER: &str = "127.0.0.1:12351";
const CLIENT: &str = "127.0.0.1:12350";

fn main() {
    App::build()
        .add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
        .add_plugin(bevy::core::CorePlugin)
        .add_plugin(ScheduleRunnerPlugin {})
        .add_plugin(NetworkingPlugin)
        .add_startup_system(setup.system())
        .add_system(print_network_events.system())
        .run();
}

fn print_network_events(
    mut reader: Local<EventReader<NetworkEvent>>,
    events: Res<Events<NetworkEvent>>
) {
    for event in reader.iter(&events) {
        match event {
            NetworkEvent::Message(conn, data) => {
                let msg = String::from_utf8_lossy(&*data);
                println!("<--- {:?} from {}", msg, conn);
            }
            NetworkEvent::Connected(conn) => println!("\tConnected: {}", conn),
            NetworkEvent::Disconnected(conn) => println!("\tDisconnected: {}", conn),
            NetworkEvent::SendError(err) => println!("\tSend Error: {}", err),
        }
    }
}

fn setup(mut net: ResMut<NetworkResource>) {
    net.bind("127.0.0.1:12351").unwrap();
}