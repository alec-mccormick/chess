pub mod info_panel;
mod sprite_interaction;
mod map;
mod input;
mod main_menu;


use bevy::prelude::*;
use crate::prelude::*;

use crate::core::{
    GameState,
    map::Tile,
    unit::{UnitStore, UnitCmd, Unit, Actions, Team, Health, is_action_valid}
};

use main_menu::MainMenuPlugin;
use info_panel::InfoPanelPlugin;


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(input::InputState::Default)
            .add_startup_system(setup.system())
            .add_plugin(MainMenuPlugin)
            .add_plugin(info_panel::InfoPanelPlugin)
            .add_plugin(map::MapUIPlugin)
            .add_system(sprite_interaction::sprite_interaction_system.system())
            .add_system(input::handle_tile_interaction.system())
            .add_system(map::handle_input_state_change.system())
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
