use bevy::{prelude::*, ecs::Command};
use std::{ops::Deref, cmp::max};
use log::{debug};
use lyon::{math::Point};

use bevy_prototype_lyon::prelude::*;

use crate::prelude::*;
use crate::core::map::{Map, Tile};
use super::utils;
use crate::render::utils::{HALF_TILE_RENDER_WIDTH_PX, HALF_TILE_RENDER_HEIGHT_PX};


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
            .add_system(handle_tile_overlay_changed.system())
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
        debug!("handle_map_spawned()");

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
            transform: Transform::from_scale(Vec3::splat(2.5)),
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
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Position, Added<Tile>)>,
) {
    for (entity, _position, tile) in query.iter() {
        debug!("handle_tile_spawned()");

        let material = materials.get_material(tile);
        commands.add_command(RenderTileCmd { entity, material });

        commands.insert_one(entity, TileOverlayState::Invisible);

        commands
            .spawn(primitive(
                materials.invisible.clone(),
                &mut meshes,
                // ShapeType::Circle(8.0),
                ShapeType::Quad(
                    Point::new(-HALF_TILE_RENDER_WIDTH_PX as f32, 0.0),
                    Point::new(0.0, HALF_TILE_RENDER_HEIGHT_PX as f32),
                    Point::new(HALF_TILE_RENDER_WIDTH_PX as f32, 0.0),
                    Point::new(0.0, -HALF_TILE_RENDER_HEIGHT_PX as f32)
                ),
                TessellationMode::Fill(&FillOptions::default()),
                Vec3::new(0.0, 0.0, 10.0),
            ))
            .with(TileOverlay)
            .with(Parent(entity));
    }
}

#[derive(Debug)]
pub struct TileOverlay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileOverlayState {
    Invisible,
    Visible,
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
        let z = (7 + position.x - position.y) as f32 / 14.0;
        let translation = utils::convert_position_to_vec2(position).extend(z);

        Transform::from_translation(translation)
    }
}

/// ==========================================================================
/// Tile Overlay
/// ==========================================================================
fn handle_tile_overlay_changed(
    tile_materials: Res<TileMaterials>,
    tile_query: Query<With<Tile, (Changed<TileOverlayState>, &Children)>>,
    mut overlay_query: Query<With<TileOverlay, &mut Handle<ColorMaterial>>>
) {
    for (tile_overlay_state, children) in tile_query.iter() {
        debug!("handle_tile_overlay_changed() {:?}", *tile_overlay_state);

        for child in children.iter() {
            if let Ok(mut material) = overlay_query.get_mut(*child) {
                *material = match *tile_overlay_state {
                    TileOverlayState::Invisible => tile_materials.invisible.clone(),
                    TileOverlayState::Visible => tile_materials.hover_overlay.clone()
                };
            }
        }
    }
}


/// ==========================================================================
/// Resources
/// ==========================================================================
#[derive(Debug)]
pub struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
    pub hover_overlay: Handle<ColorMaterial>,
    pub invisible: Handle<ColorMaterial>
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
            hover_overlay: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.15).into()),
            invisible: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into())
        }
    }
}