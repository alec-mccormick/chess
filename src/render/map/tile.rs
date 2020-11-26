use bevy::{prelude::*, ecs::Command};
use log::{debug};
use crate::prelude::*;

use crate::core::map::{Tile};

use super::super::utils;
use std::ops::Deref;


pub struct RenderTileCmd {
    pub entity: Entity,
    pub material: Handle<ColorMaterial>,
}

impl Command for RenderTileCmd {
    fn write(self: Box<Self>, world: &mut World, _resources: &mut Resources) {
        self.insert_mesh(world);
        self.spawn_sprite(world);
    }
}

impl RenderTileCmd {
    fn insert_mesh(&self, world: &mut World) {
        let position = world.get::<Position>(self.entity).unwrap();
        let translation = convert_position_to_translation(position);

        world.insert(self.entity, MeshComponents {
            transform: Transform::from_translation(translation),
            ..Default::default()
        });
    }

    fn spawn_sprite(&self, world: &mut World) {
        let sprite_entity = world.spawn(SpriteComponents {
            material: self.material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -16.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        world.insert_one(sprite_entity, Parent(self.entity)).unwrap();
    }
}



fn convert_position_to_translation(position: &Position) -> Vec3 {
    let z = (10 + position.x - position.y) as f32;
    utils::convert_position_to_vec2(position).extend(z)
}



/// Tile Materials
#[derive(Debug)]
pub struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl TileMaterials {
    pub(crate) fn get_material(&self, tile: impl Deref<Target=Tile>) -> Handle<ColorMaterial> {
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