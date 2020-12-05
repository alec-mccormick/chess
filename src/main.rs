use bevy::prelude::*;

use log::info;
use std::net::SocketAddr;


use chess::core::{CorePlugin, ServerBindAddr};

use chess::{render::RenderPlugin, ui::UIPlugin};

const SERVER: &str = "127.0.0.1:12351";
const CLIENT: &str = "127.0.0.1:12350";


fn main() {
    env_logger::init();

    let addr: SocketAddr = SERVER.parse().expect("Unable to parse socket address");


    App::build()
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1680,
            height: 1050,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        // .add_system_to_stage(stage::FIRST, print_frame.system())
        .add_plugins(DefaultPlugins)
        .add_resource(ServerBindAddr(addr))
        .add_plugin(CorePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn print_frame() {
    // trace!("New frame");
}

fn setup(
    // mut net: ResMut<NetworkResource>
    mut commands: Commands,
    mut events: ResMut<Events<chess::ui::CreateMainMenuEvent>>,
) {
    // net.bind("0.0.0.0:12345").unwrap();
    info!("App Setup - Spawning Main Menu");

    events.send(chess::ui::CreateMainMenuEvent);

    // commands.spawn((chess::ui::MainMenu,));
}
