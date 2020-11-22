pub mod unit;
pub mod map;

use bevy::{prelude::*};
use unit::{UnitMaterials, append_sprite_to_unit};
use map::{TileMaterials, append_sprite_to_tile};


pub struct RenderPlugin;

impl Plugin for RenderPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<UnitMaterials>()
            .init_resource::<TileMaterials>()
            .add_startup_system(setup.system())
            .add_system(append_sprite_to_unit.system())
            .add_system(append_sprite_to_tile.system())
        ;
    }
}



fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
    ;
}