use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
        .add_plugin(UIPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup_map.system())
        .add_startup_system(setup_units.system())
        .run()
    ;
}

fn setup_map(
    mut commands: Commands,
) {

    // let map = commands
    //     .spawn(MapComponents::default())
    //     .current_entity()
    //     .unwrap();
    //
    // for x in 0..=7 {
    //     for y in 0..=7 {
    //         let position = Position::new(x, y);
    //         let tile = if (x + y) % 2 == 0 { Tile::Black } else { Tile::White };
    //
    //         commands
    //             .spawn(TileComponents { tile, position })
    //             .with(Parent(map));
    //     }
    // }
    //
    // for x in 0..=7 {
    //     commands.spawn(UnitComponents { team: Team::White, position: Position::new(x, 1), ..pawn() }).with(Parent(map));
    //     commands.spawn(UnitComponents { team: Team::Black, position: Position::new(x, 6), ..pawn() }).with(Parent(map));
    // }
    //
    // let team = Team::White;
    // commands.spawn(UnitComponents { team, position: Position::new(0, 0), ..rook() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(1, 0), ..knight() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(2, 0), ..bishop() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(3, 0), ..queen() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(4, 0), ..king() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(5, 0), ..bishop() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(6, 0), ..knight() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(7, 0), ..rook() }).with(Parent(map));
    //
    //
    // let team = Team::Black;
    // commands.spawn(UnitComponents { team, position: Position::new(0, 7), ..rook() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(1, 7), ..knight() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(2, 7), ..bishop() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(3, 7), ..queen() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(4, 7), ..king() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(5, 7), ..bishop() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(6, 7), ..knight() }).with(Parent(map));
    // commands.spawn(UnitComponents { team, position: Position::new(7, 7), ..rook() }).with(Parent(map));
}

fn setup_units(mut commands: Commands, query: Query<(Entity, &Map)>) {
    println!("SETUP UNITS");

    for (entity, map) in query.iter() {
        println!("MAP FOUND");
    }


}