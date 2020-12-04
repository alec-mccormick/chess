use bevy::prelude::*;
use crate::prelude::*;

mod tile;
mod map;



pub use map::*;
pub use tile::*;



// ==========================================================================
// Plugin
// ==========================================================================
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


// ==========================================================================
// MapSpawner
// ==========================================================================
#[derive(Default)]
pub struct MapSpawner;


impl EntitySpawner for MapSpawner {

    fn spawn<'a>(&self, commands: &'a mut Commands) -> &'a mut Commands {
        commands
            .spawn(MapComponents::default())
            .with_children(|commands| {
                for x in 0..=7 {
                    for y in 0..=7 {
                        let position = Position::new(x, y);
                        let tile = if (x + y) % 2 == 0 { Tile::Black } else { Tile::White };

                        let mut labels = Labels::default();
                        labels.insert(format!("tile:{},{}", position.x, position.y));

                        commands.spawn(TileComponents { tile, position, labels });
                    }
                }
            })
    }
}


