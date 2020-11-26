mod map;
mod tile;

use bevy::{prelude::*};


#[derive(Debug, Clone)]
pub struct RenderMapPlugin;

impl Plugin for RenderMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<tile::TileMaterials>()
            .add_system(map::render_map.system())
            .add_system(tile::render_tile.system())
        ;
    }
}