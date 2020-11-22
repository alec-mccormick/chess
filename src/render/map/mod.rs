use bevy::{prelude::*};
use crate::prelude::*;

use crate::core::map::{Tile};

pub struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl FromResources for TileMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        TileMaterials {
            white: materials.add(asset_server.load("white.png").into()),
            black: materials.add(asset_server.load("black.png").into()),
        }
    }
}

fn get_material_for_tile(materials: &Res<TileMaterials>, tile: &Tile) -> Handle<ColorMaterial> {
    match tile {
        Tile::White => materials.white.clone(),
        Tile::Black => materials.black.clone()
    }
}

pub fn append_sprite_to_tile(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    query: Query<Without<Sprite, (Entity, &Tile, &Position)>>
) {
    for (entity, tile, position) in query.iter() {
        // println!("append sprite to tile! {:?} {:?}", entity, position);

        let material = get_material_for_tile(&materials, tile);

        let transform = Transform {
            translation: convert_position_to_translation(position),
            rotation: Quat::identity(),
            scale: Vec3::splat(0.5)
        };

        commands.insert(entity, SpriteComponents {
            material,
            transform,
            ..Default::default()
        });
    }
}


fn convert_position_to_translation(position: &Position) -> Vec3 {
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;
    Vec3::new(x, y, 0.0)
}