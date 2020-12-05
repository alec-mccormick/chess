use crate::prelude::*;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

mod map;
mod tile;


pub use map::*;
pub use tile::*;


// ==========================================================================
// Plugin
// ==========================================================================
#[derive(Debug, Clone)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileStore::default())
            .add_system(TileStore::handle_position_changed.system());
    }
}


// ==========================================================================
// MapSpawner
// ==========================================================================
#[derive(Debug, Serialize, Deserialize)]
pub struct MapDescriptor {
    dimensions: Dimensions,
    tiles: Vec<(Tile, Position)>,
}

impl EntitySpawner for MapDescriptor {
    fn spawn(self, commands: &mut Commands) -> &mut Commands {
        let MapDescriptor { dimensions, tiles } = self;

        let map = MapComponents {
            dimensions,
            ..Default::default()
        };

        commands.spawn(map).with_children(|commands| {
            for (tile, position) in tiles.into_iter() {
                let mut labels = Labels::default();
                labels.insert(format!("tile:{},{}", position.x, position.y));

                commands.spawn(TileComponents { tile, position, labels });
            }
        })
    }
}

impl Default for MapDescriptor {
    fn default() -> Self {
        let dimensions = Dimensions { width: 8, height: 8 };
        let mut tiles = Vec::new();

        for x in 0..=7 {
            for y in 0..=7 {
                let position = Position::new(x, y);
                let tile = if (x + y) % 2 == 0 { Tile::Black } else { Tile::White };

                tiles.push((tile, position));
            }
        }

        MapDescriptor { dimensions, tiles }
    }
}