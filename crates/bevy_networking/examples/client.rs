use bevy::app::{ScheduleRunnerPlugin};
use bevy::prelude::*;

use std::net::SocketAddr;
use std::time::Duration;

use bevy_networking::{
    NetworkDelivery, NetworkResource, NetworkingPlugin,
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
        .init_resource::<SendTimer>()
        .add_system(send_messages.system())
        .run();
}


fn setup(mut net: ResMut<NetworkResource>) {
    net.bind("127.0.0.1:12350").unwrap();
}

fn send_messages(
    time: Res<Time>,
    mut state: ResMut<SendTimer>,
    net: ResMut<NetworkResource>,
) {
    state.message_timer.tick(time.delta_seconds);
    if state.message_timer.finished {
        let server: SocketAddr = SERVER.parse().unwrap();

        let msg = "Hello from client!";
        println!("---> {:?}", msg);

        net.send(
            server,
            msg.as_bytes(),
            NetworkDelivery::ReliableSequenced(Some(1)),
        ).unwrap();



        state.message_timer.reset();
    }
}



struct SendTimer {
    message_timer: Timer,
}

impl Default for SendTimer {
    fn default() -> Self {
        SendTimer {
            message_timer: Timer::from_seconds(3.0, true),
        }
    }
}