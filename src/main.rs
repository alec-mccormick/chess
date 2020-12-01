use bevy::prelude::*;
// use bevy_prototype_networking_laminar::{NetworkingPlugin, NetworkResource};

use log::{info};



use chess::core::{CorePlugin};

use chess::render::RenderPlugin;
use chess::ui::UIPlugin;




fn main() {
    env_logger::init();

    App::build()
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1680,
            height: 1050,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_system_to_stage(stage::FIRST, print_frame.system())
        .add_plugins(DefaultPlugins)
        // .add_plugin(NetworkingPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup.system())
        .run()
    ;
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