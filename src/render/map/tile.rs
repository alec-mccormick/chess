use bevy::prelude::*;
use log::{debug};
use crate::prelude::*;

use crate::core::map::{Tile};

use super::super::utils;
use std::ops::Deref;


#[derive(Debug)]
pub struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl TileMaterials {
    fn get_material(&self, tile: impl Deref<Target=Tile>) -> Handle<ColorMaterial> {
        match tile.deref() {
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



pub fn render_tile(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    query: Query<(Entity, Added<Tile>, &Position)>
) {
    for (entity, tile, position) in query.iter() {
        let material = materials.get_material(tile);

        let translation = convert_position_to_translation(position);

        commands.insert(entity, MeshComponents {
            transform: Transform::from_translation(translation),
            ..Default::default()
        });

        commands
            .spawn(SpriteComponents {
                material,
                transform: Transform {
                    translation: Vec3::new(0.0, -16.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Parent(entity));
    }
}

fn convert_position_to_translation(position: &Position) -> Vec3 {
    let z = (10 + position.x - position.y) as f32;
    utils::convert_position_to_vec2(position).extend(z)
}