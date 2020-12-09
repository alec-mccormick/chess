use crate::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use log::{debug, trace};
use lyon::math::Point;
use serde::{Deserialize, Serialize};
use std::{
    cmp::max,
    collections::HashMap,
    ops::Deref
};
use strum::Display;


#[derive(Debug, Bundle)]
pub struct TileComponents {
    pub tile: Tile,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum Tile {
    Black,
    White,
}


pub struct TileOverlayEntity(Entity);

pub struct TileOverlay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileOverlayState {
    Invisible,
    Visible,
}


fn tile_overlay_sprite(
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>
) -> SpriteComponents {
    primitive(
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
    )
}


// ==========================================================================
// --- Systems
// ==========================================================================
pub fn handle_tile_spawned(
    mut commands: Commands,
    materials: Res<TileMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, Added<Tile>)>,
) {
    for (entity, tile) in query.iter() {
        trace!("handle_tile_spawned() - Add sprite to tile");

        let material = materials.get_material(tile);

        let tile_sprite_entity = commands
            .spawn(SpriteComponents {
                material,
                transform: Transform::from_translation(Vec3::new(0.0, -16.0, 0.0)), // offset sprite 16 px down
                ..Default::default()
            })
            .current_entity()
            .unwrap();

        let tile_overlay_entity = commands
            .spawn(tile_overlay_sprite(materials.invisible.clone(), &mut meshes))
            .with(TileOverlay)
            .current_entity()
            .unwrap();

        commands
            .insert(entity, (
                TileOverlayEntity(tile_overlay_entity.clone()),
                TileOverlayState::Invisible,
                Interaction::None
            ))
            .insert(entity, MeshComponents::default())
            .push_children(entity, &[tile_overlay_entity, tile_sprite_entity]);
    }
}

pub fn handle_tile_overlay_state_change(
    tile_materials: Res<TileMaterials>,
    tile_query: Query<With<Tile, (Changed<TileOverlayState>, &TileOverlayEntity)>>,
    mut overlay_query: Query<With<TileOverlay, &mut Handle<ColorMaterial>>>,
) {
    for (tile_overlay_state, tile_overlay_entity) in tile_query.iter() {
        debug!("handle_tile_overlay_stage_change() {:?}", *tile_overlay_state);

        if let Ok(mut material) = overlay_query.get_mut(tile_overlay_entity.0) {
            *material = match *tile_overlay_state {
                TileOverlayState::Invisible => tile_materials.invisible.clone(),
                TileOverlayState::Visible => tile_materials.hover_overlay.clone(),
            };
        }
    }
}


// ==========================================================================
// --- Resources
// ==========================================================================
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
