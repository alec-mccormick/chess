mod battle;
mod prelude;
mod units;

use battle::BattlePlugin;
use bevy::prelude::*;



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
        // .add_plugins(MinimalPlugins)
        .add_plugin(BattlePlugin)
        .run();
}