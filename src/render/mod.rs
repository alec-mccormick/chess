pub mod unit;
pub mod map;
mod utils;

use bevy::{prelude::*};
use crate::prelude::*;
use unit::{UnitMaterials, append_sprite_to_unit};


pub struct RenderPlugin;

impl Plugin for RenderPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(map::RenderMapPlugin)
            .init_resource::<UnitMaterials>()
            .add_startup_system(setup.system())
            // .add_system(append_sprite_to_unit.system())
            .add_system(handle_position_update.system())
        ;
    }
}



fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
    ;
}


fn handle_position_update(
    mut query: Query<With<Sprite, (Mutated<Position>, &mut Transform)>>
) {
    for (position, mut transform) in query.iter_mut() {
        let translate = &transform.translation;

        transform.translation = utils::convert_position_to_vec2(&*position)
            .extend(translate.z());
    }
}