use bevy::prelude::*;
// use bevy_prototype_networking_laminar::{NetworkingPlugin, NetworkResource};


use chess::prelude::*;

use chess::core::{
    CorePlugin,
    map::{TileComponents, Tile, MapComponents, Map},
    unit::{UnitComponents, Unit, Team, Health, Actions},
};

use chess::render::RenderPlugin;
use chess::ui::{UIPlugin};

use chess::units::*;

/// Todo:
/// x GameState, swap turns with each move (needs to be fixed to not work on invalid moves)
/// x Validate move before executing
/// - Highlight moves
/// x Show current player with text
/// x Implement attacking
///     x Handle health changes in various modules (despawn + remove sprite)
/// x Implement Knight
/// x Implement King
/// x Add robust logging
/// x Update rendering to isometric
/// - Add Networking
/// - Migrate to using SpriteConfig
/// - Add Bot AI
/// x Add startup screen
///     - Allow user to select team
/// - Build Action UI
///     - Add Icon
///     - Add Description
/// - Add State Machine
/// - Add events for action executed, etc.
/// - Update Action to no longer rely on Query
///     - Maintain internal list of all units in UnitStore?
///     - Migrate to ObjectId


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
        .add_plugins(DefaultPlugins)
        // .add_plugin(NetworkingPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(UIPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup.system())
        .run()
    ;
}

fn setup(
    // mut net: ResMut<NetworkResource>
) {
    // net.bind("0.0.0.0:12345").unwrap();
}