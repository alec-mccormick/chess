use bevy::{prelude::*, ecs::Command};
use std::{ops::Deref, cmp::max};
use log::{debug};

use crate::prelude::*;
use crate::core::map::{Map, Tile};
use super::utils;


/// ==========================================================================
/// Plugin
/// ==========================================================================
pub struct RenderMapPlugin;

impl Plugin for RenderMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<TileMaterials>()
            .add_system(handle_map_spawned.system())
            .add_system(handle_tile_spawned.system())
        ;
    }
}


/// ==========================================================================
/// Map Rendering
/// ==========================================================================
pub fn handle_map_spawned(
    mut commands: Commands,
    query: Query<(Entity, &Dimensions, Added<Map>)>,
) {
    for (entity, _dimensions, _map) in query.iter() {
        commands.add_command(RenderMapCmd { entity });
    }
}

/// Element to contain map and control map scale.
#[derive(Clone, Copy)]
pub struct MapContainer;

/// Command to make a draw a map & position it in the screen.
#[derive(Clone, Copy)]
struct RenderMapCmd {
    entity: Entity,
}

impl Command for RenderMapCmd {
    fn write(self: Box<Self>, world: &mut World, _: &mut Resources) {
        let map_container_entity = self.spawn_container(world);
        self.insert_map_mesh(world, map_container_entity);
    }
}

impl RenderMapCmd {
    fn spawn_container(&self, world: &mut World) -> Entity {
        let entity = world.spawn(MeshComponents {
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        });

        world.insert_one(entity, MapContainer).unwrap();
        entity
    }

    fn insert_map_mesh(&self, world: &mut World, parent: Entity) {
        let dimensions = world.get::<Dimensions>(self.entity).unwrap();

        let translation = utils::convert_dimensions_to_map_offset(dimensions);

        world.insert(self.entity, MeshComponents {
            transform: Transform::from_translation(translation),
            ..Default::default()
        }).unwrap();

        world.insert_one(self.entity, Parent(parent)).unwrap();
    }
}



/// ==========================================================================
/// Tile Rendering
/// ==========================================================================
fn handle_tile_spawned(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    query: Query<(Entity, &Position, Added<Tile>)>,
) {
    for (entity, _position, tile) in query.iter() {
        let material = materials.get_material(tile);
        commands.add_command(RenderTileCmd { entity, material });
    }
}

struct RenderTileCmd {
    entity: Entity,
    material: Handle<ColorMaterial>,
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

        world.insert(self.entity, MeshComponents {
            transform: Self::generate_transform(position),
            ..Default::default()
        }).unwrap();
    }

    fn spawn_sprite(&self, world: &mut World) {
        let sprite_entity = world.spawn(SpriteComponents {
            material: self.material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -16.0, 0.0),    // offset sprite 16 px down
                ..Default::default()
            },
            ..Default::default()
        });

        world.insert_one(sprite_entity, Parent(self.entity)).unwrap();
    }

    fn generate_transform(position: &Position) -> Transform {
        // todo: update to use max dimension of map entity instead
        let z = (8 + position.x - position.y) as f32 / 16.0;
        let translation = utils::convert_position_to_vec2(position).extend(z);

        Transform::from_translation(translation)
    }
}


/// ==========================================================================
/// Resources
/// ==========================================================================
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