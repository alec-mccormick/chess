use crate::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use log::{debug, trace};

mod tile;


pub use tile::*;
use crate::core::AppConfig;


pub struct GameCamera;

// ==========================================================================
// MapSpawner
// ==========================================================================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapDescriptor {
    dimensions: Dimensions,
    tiles: Vec<(Tile, Position)>,
}

impl SpawnWithCommands for MapDescriptor {
    fn spawn_with_commands(self, commands: &mut Commands) -> &mut Commands {
        let MapDescriptor { dimensions, tiles } = self;

        let map = MapComponents {
            dimensions,
            ..Default::default()
        };

        commands.spawn(map)
            .with_children(|commands| {
            for (tile, position) in tiles.into_iter() {
                commands.spawn(TileComponents { tile, position });
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


/// ==========================================================================
/// Map Rendering
/// ==========================================================================
pub fn handle_map_spawned(
    mut commands: Commands,
    app_config: Res<AppConfig>,
    query: Query<(Entity, &Dimensions, Added<Map>)>
) {
    for (entity, dimensions, _map) in query.iter() {
        debug!("handle_map_spawned() - Insert Mesh components for rendering");

        let scale = Vec3::splat(app_config.scale * 2.5);
        let translation = convert_dimensions_to_map_offset(dimensions) * scale;

        commands.insert(
            entity,
            MeshComponents {
                transform: Transform {
                    translation,
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
    }
}

fn convert_dimensions_to_map_offset(dimensions: &Dimensions) -> Vec3 {
    let x = ((dimensions.width + dimensions.height - 2) * HALF_TILE_RENDER_WIDTH_PX) as f32;
    Vec3::new(-x / 2.0, 0.0, 0.0)
}


#[derive(Debug, Bundle)]
pub struct MapComponents {
    pub map: Map,
    pub dimensions: Dimensions,
}

impl Default for MapComponents {
    fn default() -> Self {
        MapComponents {
            map: Map,
            dimensions: Dimensions::new(8, 8),
        }
    }
}

#[derive(Debug)]
pub struct Map;