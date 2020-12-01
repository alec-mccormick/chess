pub mod unit;
pub mod map;
pub(crate) mod utils;

use bevy::{prelude::*};
use crate::prelude::*;

use log::{trace};



pub struct RenderPlugin;

impl Plugin for RenderPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(map::RenderMapPlugin)
            .add_plugin(unit::RenderUnitPlugin)
            .add_startup_system(setup.system())
            .add_system_to_stage(stage::POST_UPDATE, handle_position_update.system())
        ;
    }
}



fn setup(mut commands: Commands) {
    let mut camera = Camera2dComponents::default();
    // camera.transform.scale = Vec3::splat(0.8);

    commands
        .spawn(camera)
        .with(map::GameCamera)
    ;
    // let map_offset = (7 * utils::HALF_TILE_RENDER_HEIGHT_PX) as f32 * 2.5;
    // let start = (window.width as f32 / 2.0) - map_offset - (2 * utils::HALF_TILE_RENDER_HEIGHT_PX as f32);
    // println!("!!! Window width: {}, map_offset: {}, start: {}", window.width, map_offset, start);
}


fn handle_position_update(
    mut query: Query<(Changed<Position>, &mut Transform, Option<&TransformOffset>)>
) {
    for (position, mut transform, transform_offset) in query.iter_mut() {
        trace!("handle_position_update() - Update x,y translation: {:?}", *position);

        let z = (7 + position.x - position.y) as f32 / 14.0;

        let translation = utils::convert_position_to_vec2(&*position).extend(z);

        transform.translation = match transform_offset {
            Some(offset) => translation + offset.0,
            None => translation
        };
    }
}