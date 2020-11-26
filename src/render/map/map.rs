use bevy::prelude::*;
use log::{debug};
use crate::prelude::*;

use crate::core::map::{Map};
use std::cmp::max;

use super::super::utils::{HALF_TILE_RENDER_WIDTH_PX};


pub fn render_map(
    mut commands: Commands,
    query: Query<(Entity, &Dimensions, Added<Map>)>,
) {
    for (entity, dimensions, map) in query.iter() {
        debug!("Map added: {:?}, {:?}", *map, dimensions);

        commands.insert(entity, MeshComponents {
            transform: Transform {
                translation: convert_dimensions_to_translation(dimensions),
                scale: Vec3::splat(2.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}


fn convert_dimensions_to_translation(dimensions: &Dimensions) -> Vec3 {
    let x = ((dimensions.width + dimensions.height - 2) * HALF_TILE_RENDER_WIDTH_PX) as f32;
    Vec3::new(-x, 0.0, 0.0)
}