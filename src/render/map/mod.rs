use bevy::{ecs::Command, prelude::*, render::camera::Camera};
use log::{debug, trace};
use lyon::math::Point;
use std::{cmp::max, ops::Deref};

use bevy_prototype_lyon::prelude::*;

use super::utils::{self, HALF_TILE_RENDER_HEIGHT_PX, HALF_TILE_RENDER_WIDTH_PX};
use crate::{
    core::{AppConfig, map::{Map, Tile}},
    prelude::*,
};


/// ==========================================================================
/// Plugin
/// ==========================================================================
pub struct RenderMapPlugin;

impl Plugin for RenderMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileMaterials>()
            .add_system(handle_map_spawned.system())
            .add_system(handle_tile_spawned.system())
            .add_system_to_stage(stage::UPDATE, TileOverlay::handle_state_changed.system());
    }
}


pub struct GameCamera;

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
        let translation = utils::convert_dimensions_to_map_offset(dimensions) * scale;

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


/// ==========================================================================
/// Tile Rendering
/// ==========================================================================
fn handle_tile_spawned(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, Added<Tile>)>,
) {
    for (entity, tile) in query.iter() {
        trace!("handle_tile_spawned() - Add sprite to tile");

        let material = materials.get_material(tile);

        commands
            .insert(entity, MeshComponents::default())
            .insert(entity, (TileOverlayState::Invisible, Interaction::None));

        TileOverlay::spawn(&mut commands, &mut meshes, materials.invisible.clone(), entity);

        commands
            .spawn(SpriteComponents {
                material,
                transform: Transform {
                    translation: Vec3::new(0.0, -16.0, 0.0), // offset sprite 16 px down
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Parent(entity));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileOverlayState {
    Invisible,
    Visible,
}

/// ==========================================================================
/// Tile Overlay
/// ==========================================================================
pub struct TileOverlayComponents {
    overlay: TileOverlay,
}

#[derive(Debug)]
pub struct TileOverlay;


impl TileOverlay {
    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<ColorMaterial>,
        parent: Entity,
    ) {
        trace!("TileOverlay::spawn()");

        commands
            .spawn(primitive(
                material,
                meshes,
                ShapeType::Quad(
                    Point::new(-HALF_TILE_RENDER_WIDTH_PX as f32, 0.0),
                    Point::new(0.0, HALF_TILE_RENDER_HEIGHT_PX as f32),
                    Point::new(HALF_TILE_RENDER_WIDTH_PX as f32, 0.0),
                    Point::new(0.0, -HALF_TILE_RENDER_HEIGHT_PX as f32),
                ),
                TessellationMode::Fill(&FillOptions::default()),
                Vec3::new(0.0, 0.0, 0.1),
            ))
            .with(TileOverlay)
            .with(Parent(parent));
    }

    fn handle_state_changed(
        tile_materials: Res<TileMaterials>,
        tile_query: Query<With<Tile, (Changed<TileOverlayState>, &Children)>>,
        mut overlay_query: Query<With<TileOverlay, &mut Handle<ColorMaterial>>>,
    ) {
        for (tile_overlay_state, children) in tile_query.iter() {
            debug!("TileOverlay::handle_tile_overlay_changed() {:?}", *tile_overlay_state);

            for child in children.iter() {
                if let Ok(mut material) = overlay_query.get_mut(*child) {
                    *material = match *tile_overlay_state {
                        TileOverlayState::Invisible => tile_materials.invisible.clone(),
                        TileOverlayState::Visible => tile_materials.hover_overlay.clone(),
                    };
                }
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
    pub invisible: Handle<ColorMaterial>,
}

impl TileMaterials {
    pub(crate) fn get_material(&self, tile: impl Deref<Target = Tile>) -> Handle<ColorMaterial> {
        match tile.deref() {
            Tile::White => self.white.clone(),
            Tile::Black => self.black.clone(),
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
            invisible: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
        }
    }
}
