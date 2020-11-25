use bevy::prelude::*;
use chess::prelude::*;

use chess::core::unit::{UnitComponents, Unit, Team, Health, Actions};
use chess::core::CorePlugin;

use chess::render::RenderPlugin;
use chess::ui::UIPlugin;

use chess::units::*;

/// Todo:
/// x GameState, swap turns with each move (needs to be fixed to not work on invalid moves)
/// x Validate move before executing
/// - Highlight moves
/// x Show current player with text
/// x Implement attacking
///     x Handle health changes in various modules (despawn + remove sprite)
/// - Implement Knight
/// - Implement King
/// - Add robust logging
/// - Update rendering to isometric
/// - Migrate to using SpriteConfig
/// - Add Bot UI
/// - Add startup screen
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
        .add_plugin(CorePlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.system())
        .run()
    ;
}


fn setup(mut commands: Commands) {


    for x in 0..8 {
        commands.spawn(UnitComponents { team: Team::White, position: Position::new(x, 1), ..pawn() });
        commands.spawn(UnitComponents { team: Team::Black, position: Position::new(x, 6), ..pawn() });
    }

    let team = Team::White;

    commands.spawn(UnitComponents { team, position: Position::new(0, 0), ..rook() });
    commands.spawn(UnitComponents { team, position: Position::new(2, 0), ..bishop() });
    commands.spawn(UnitComponents { team, position: Position::new(3, 0), ..queen() });
    commands.spawn(UnitComponents { team, position: Position::new(5, 0), ..bishop() });
    commands.spawn(UnitComponents { team, position: Position::new(7, 0), ..rook() });


    let team = Team::Black;

    commands.spawn(UnitComponents { team, position: Position::new(0, 7), ..ro ok() });
    commands.spawn(UnitComponents { team, position: Position::new(2, 7), ..bishop() });
    commands.spawn(UnitComponents { team, position: Position::new(3, 7), ..queen() });
    commands.spawn(UnitComponents { team, position: Position::new(5, 7), ..bishop() });
    commands.spawn(UnitComponents { team, position: Position::new(7, 7), ..rook() });
}