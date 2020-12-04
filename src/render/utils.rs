use crate::prelude::*;
use bevy::prelude::*;


pub const TILE_RENDER_WIDTH_PX: i32 = 56;
pub const TILE_RENDER_HEIGHT_PX: i32 = 42;

pub const HALF_TILE_RENDER_WIDTH_PX: i32 = TILE_RENDER_WIDTH_PX / 2;
pub const HALF_TILE_RENDER_HEIGHT_PX: i32 = TILE_RENDER_HEIGHT_PX / 2;


pub fn convert_position_to_vec2(pos: &Position) -> Vec2 {
    let x = ((pos.y + pos.x) * HALF_TILE_RENDER_WIDTH_PX) as f32;
    let y = ((pos.y - pos.x) * HALF_TILE_RENDER_HEIGHT_PX) as f32;
    Vec2::new(x, y)
}

pub fn convert_dimensions_to_map_offset(dimensions: &Dimensions) -> Vec3 {
    let x = ((dimensions.width + dimensions.height - 2) * HALF_TILE_RENDER_WIDTH_PX) as f32;
    Vec3::new(-x / 2.0, 0.0, 0.0)
}
