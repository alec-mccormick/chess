mod store;
pub mod components;
pub mod cmd;
pub mod action;

pub use components::{UnitType, UnitHealth, UnitTeam, UnitComponentsPlugin};
pub use store::{UnitStore, UnitStorePlugin};
pub use action::{UnitAction, UnitActions, UnitActionPlugin};
pub use cmd::{UnitCmd, UnitCmdPlugin};


use bevy::prelude::*;



#[derive(Debug, Default)]
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(UnitComponentsPlugin)
            .add_plugin(UnitActionPlugin)
            .add_plugin(UnitCmdPlugin)
            .add_plugin(UnitStorePlugin)
        ;
    }
}
