use bevy::prelude::*;
use crate::prelude::*;


pub const TILE_RENDER_WIDTH_PX: i32 = 56;
pub const TILE_RENDER_HEIGHT_PX: i32 = 42;

pub const HALF_TILE_RENDER_WIDTH_PX: i32 = TILE_RENDER_WIDTH_PX / 2;
pub const HALF_TILE_RENDER_HEIGHT_PX: i32 = TILE_RENDER_HEIGHT_PX / 2;



pub fn convert_position_to_vec2(pos: &Position) -> Vec2 {
    let x = ((pos.y + pos.x) * HALF_TILE_RENDER_WIDTH_PX) as f32;
    let y = ((pos.y - pos.x) * HALF_TILE_RENDER_HEIGHT_PX) as f32;
    Vec2::new(x, y)
}