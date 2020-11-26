mod map;
mod tile;

use bevy::prelude::*;
use crate::prelude::*;
use crate::core::map::{Map, Tile};

use map::{RenderMapCmd};
use tile::{RenderTileCmd, TileMaterials};


#[derive(Debug, Clone)]
pub struct RenderMapPlugin;

impl Plugin for RenderMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<TileMaterials>()
            .add_system(handle_map_spawned.system())
            .add_system(handle_tile_spawned.system())
        ;
    }
}


/// ==========================================================================
/// Systems
/// ==========================================================================
pub fn handle_map_spawned(
    mut commands: Commands,
    query: Query<(Entity, &Dimensions, Added<Map>)>,
) {
    for (entity, _dimensions, _map) in query.iter() {
        commands.add_command(RenderMapCmd { entity });
    }
}

pub fn handle_tile_spawned(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    query: Query<(Entity, &Position, Added<Tile>)>,
) {
    for (entity, _position, tile) in query.iter() {
        let material = materials.get_material(tile);
        commands.add_command(RenderTileCmd { entity, material });
    }
}