use bevy::prelude::*;
use chess::prelude::*;

use chess::core::map::MapPlugin;
use chess::core::unit::{UnitPlugin, UnitComponents, Unit, Team, Health, Actions};

use chess::render::RenderPlugin;
use chess::ui::UIPlugin;
use chess::units::pawn::PawnMoveAction;


fn main() {

    App::build()
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1680,
            height: 1050,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(UnitPlugin)
        .add_plugin(MapPlugin)
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
        position: Position::new(0, 0),
        actions: Actions(vec![Box::new(PawnMoveAction)]),
    });
}