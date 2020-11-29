mod tile;
mod map;


use bevy::prelude::*;

pub use map::*;
pub use tile::*;



#[derive(Debug, Clone)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(TileStore::default())
            .add_system(TileStore::handle_position_changed.system())
        ;
    }
}



