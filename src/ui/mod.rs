use bevy::prelude::*;

mod info_panel;
mod input;
mod main_menu;
mod map;
mod sprite_interaction;

use info_panel::InfoPanelPlugin;
use input::InputState;

pub use main_menu::{CreateMainMenuEvent, MainMenu};


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(stage::UPDATE, stages::SPAWN_UI)
            .add_stage_after(stages::SPAWN_UI, stages::POST_SPAWN_UI)
            .init_resource::<main_menu::MainMenuMaterials>()
            .add_event::<main_menu::CreateMainMenuEvent>()
            .add_system(main_menu::handle_create_main_menu_event.system())
            .add_system_to_stage(stage::UPDATE, main_menu::handle_main_menu_button_interaction.system())
            .add_system_to_stage(stage::UPDATE, main_menu::handle_start_button_pressed.system())
            .add_system(main_menu::handle_join_button_pressed.system())
            .init_resource::<InputState>()
            .add_startup_system(setup.system())
            .add_plugin(InfoPanelPlugin)
            .add_system_to_stage(
                stage::PRE_UPDATE,
                sprite_interaction::sprite_interaction_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, input::handle_tile_interaction.system())
            .add_system_to_stage(stage::UPDATE, map::handle_input_state_change.system());
    }
}

pub mod stages {
    pub const SPAWN_UI: &str = "spawn_ui";
    pub const POST_SPAWN_UI: &str = "post_spawn_ui";
}

fn setup(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}
