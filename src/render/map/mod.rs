use bevy::{prelude::*};
use crate::prelude::*;

use crate::core::map::{Tile};

use super::utils;

pub struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl TileMaterials {
    fn get_material(&self, tile: &Tile) -> Handle<ColorMaterial> {
        match tile {
            Tile::White => self.white.clone(),
            Tile::Black => self.black.clone()
        }
    }
}

impl FromResources for TileMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        TileMaterials {
            white: materials.add(asset_server.load("textures/ground_0.png").into()),
            black: materials.add(asset_server.load("textures/ground_burnt.png").into()),
        }
    }
}


pub fn append_sprite_to_tile(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    query: Query<Without<Sprite, (Entity, &Tile, &Position)>>
) {
    for (entity, tile, position) in query.iter() {
        let material = materials.get_material(tile);

        let transform = Transform {
            translation: convert_position_to_translation(position),
            rotation: Quat::identity(),
            scale: Vec3::splat(2.0)
        };

        commands.insert(entity, SpriteComponents {
            material,
            transform,
            ..Default::default()
        });
    }
}

fn convert_position_to_translation(position: &Position) -> Vec3 {
    let z = (10 + position.x - position.y) as f32;
    utils::convert_position_to_vec2(position).extend(z)
}