use crate::prelude::*;
use bevy::{ecs::Command, prelude::*};

use crate::core::unit::{Team, Unit};

use super::utils;

/// ==========================================================================
/// Plugin
/// ==========================================================================
pub struct RenderUnitPlugin;

impl Plugin for RenderUnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<UnitMaterials>()
            .add_system(handle_unit_spawned.system());
    }
}

/// ==========================================================================
/// Unit Rendering
/// ==========================================================================
fn handle_unit_spawned(
    mut commands: Commands,
    materials: Res<UnitMaterials>,
    query: Query<(Entity, Added<Unit>, &Team, &Position)>,
) {
    for (entity, unit, team, _position) in query.iter() {
        let material = materials.get_unit_material(&*unit, team);

        commands
            .insert(
                entity,
                SpriteComponents {
                    material,
                    ..Default::default()
                },
            )
            .insert_one(entity, TransformOffset(Vec3::new(0.0, 16.0, 2.0)));
    }
}


/// ==========================================================================
/// Resources
/// ==========================================================================
pub struct UnitMaterials {
    white_pawn: Handle<ColorMaterial>,
    white_bishop: Handle<ColorMaterial>,
    white_knight: Handle<ColorMaterial>,
    white_rook: Handle<ColorMaterial>,
    white_king: Handle<ColorMaterial>,
    white_queen: Handle<ColorMaterial>,
    black_pawn: Handle<ColorMaterial>,
    black_bishop: Handle<ColorMaterial>,
    black_knight: Handle<ColorMaterial>,
    black_rook: Handle<ColorMaterial>,
    black_king: Handle<ColorMaterial>,
    black_queen: Handle<ColorMaterial>,
}

impl UnitMaterials {
    fn get_unit_material(&self, unit: &Unit, team: &Team) -> Handle<ColorMaterial> {
        match (unit, team) {
            (Unit::Pawn, Team::White) => self.white_pawn.clone(),
            (Unit::Pawn, Team::Black) => self.black_pawn.clone(),
            (Unit::Bishop, Team::White) => self.white_bishop.clone(),
            (Unit::Bishop, Team::Black) => self.black_bishop.clone(),
            (Unit::Rook, Team::White) => self.white_rook.clone(),
            (Unit::Rook, Team::Black) => self.black_rook.clone(),
            (Unit::Queen, Team::White) => self.white_queen.clone(),
            (Unit::Queen, Team::Black) => self.black_queen.clone(),
            (Unit::Knight, Team::White) => self.white_knight.clone(),
            (Unit::Knight, Team::Black) => self.black_knight.clone(),
            (Unit::King, Team::White) => self.white_king.clone(),
            (Unit::King, Team::Black) => self.black_king.clone(),
        }
    }
}

impl FromResources for UnitMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        UnitMaterials {
            white_pawn: materials.add(asset_server.load("whitePawn.png").into()),
            white_bishop: materials.add(asset_server.load("whiteBishop.png").into()),
            white_knight: materials.add(asset_server.load("whiteKnight.png").into()),
            white_rook: materials.add(asset_server.load("whiteRook.png").into()),
            white_king: materials.add(asset_server.load("whiteKing.png").into()),
            white_queen: materials.add(asset_server.load("whiteQueen.png").into()),
            black_pawn: materials.add(asset_server.load("blackPawn.png").into()),
            black_bishop: materials.add(asset_server.load("blackBishop.png").into()),
            black_knight: materials.add(asset_server.load("blackKnight.png").into()),
            black_rook: materials.add(asset_server.load("blackRook.png").into()),
            black_king: materials.add(asset_server.load("blackKing.png").into()),
            black_queen: materials.add(asset_server.load("blackQueen.png").into()),
        }
    }
}
