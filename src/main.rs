mod battle;
mod prelude;
mod unit;

use battle::BattlePlugin;
use bevy::prelude::*;



fn main() {
    App::build()
        // .add_plugins(DefaultPlugins)
        .add_plugins(MinimalPlugins)
        .add_plugin(BattlePlugin::default())
        .run();
}