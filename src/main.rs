use bevy::prelude::*;
use chess::prelude::*;

use chess::core::unit::{UnitComponents, Unit, Team, Health, Actions};
use chess::core::CorePlugin;

use chess::render::RenderPlugin;
use chess::ui::UIPlugin;
use chess::units::pawn::PawnMoveAction;


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

    commands.spawn(UnitComponents {
        unit: Unit::Pawn,
        team: Team::White,
        health: Health(1),
        position: Position::new(0, 1),
        actions: Actions(vec![Box::new(PawnMoveAction)]),
    });

    commands.spawn(UnitComponents {
        unit: Unit::Pawn,
        team: Team::Black,
        health: Health(1),
        position: Position::new(1, 6),
        actions: Actions(vec![Box::new(PawnMoveAction)]),
    });
}