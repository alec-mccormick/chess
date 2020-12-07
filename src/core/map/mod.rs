use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod map;
mod tile;


pub use map::*;
pub use tile::*;


// ==========================================================================
// MapSpawner
// ==========================================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
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
