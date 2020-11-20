mod store;
pub mod components;
pub mod cmd;
pub mod action;

pub use components::{UnitType, UnitHealth, UnitTeam, UnitComponentsPlugin};
pub use store::{UnitStore, UnitStorePlugin};
pub use action::{UnitAction, UnitActions, UnitActionPlugin};
pub use cmd::{UnitCmd, UnitCmdPlugin};


use bevy::{prelude::*, app::PluginGroupBuilder};

#[derive(Bundle)]
pub struct UnitComponents {
    pub unit_type: UnitType,
    pub health: UnitHealth,
    pub team: UnitTeam,
    pub actions: UnitActions,
}


#[derive(Debug, Default)]
pub struct UnitPlugins;

impl PluginGroup for UnitPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(UnitComponentsPlugin)
            .add(UnitActionPlugin)
            .add(UnitCmdPlugin)
            .add(UnitStorePlugin)
        ;
    }
}
