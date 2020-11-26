mod tile;
mod map;


use bevy::prelude::*;

pub use map::*;
pub use tile::*;



#[derive(Debug, Clone)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        // app
        //     .add_startup_system(setup_map.system())
        // ;
    }
}



