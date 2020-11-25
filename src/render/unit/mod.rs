use bevy::{prelude::*};
use crate::prelude::*;

use crate::core::unit::{Unit, Team};



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

fn get_material_for_unit(materials: &Res<UnitMaterials>, unit: &Unit, team: &Team) -> Handle<ColorMaterial> {
    match (unit, team) {
        (Unit::Pawn, Team::White) => materials.white_pawn.clone(),
        (Unit::Pawn, Team::Black) => materials.black_pawn.clone(),
        (Unit::Bishop, Team::White) => materials.white_bishop.clone(),
        (Unit::Bishop, Team::Black) => materials.black_bishop.clone(),
        (Unit::Rook, Team::White) => materials.white_rook.clone(),
        (Unit::Rook, Team::Black) => materials.black_rook.clone(),
        (Unit::Queen, Team::White) => materials.white_queen.clone(),
        (Unit::Queen, Team::Black) => materials.black_queen.clone(),
        (Unit::Knight, Team::White) => materials.white_knight.clone(),
        (Unit::Knight, Team::Black) => materials.black_knight.clone(),
        (Unit::King, Team::White) => materials.white_king.clone(),
        (Unit::King, Team::Black) => materials.black_king.clone(),
        // _ => {}
    }
}

pub fn append_sprite_to_unit(
    mut commands: Commands,
    materials: Res<UnitMaterials>,
    query: Query<Without<Sprite, (Entity, &Unit, &Position, &Team)>>,
) {
    // println!("Append sprite!");

    for (entity, unit, position, team) in query.iter() {
        let material = get_material_for_unit(&materials, unit, team);

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
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;
    Vec3::new(x, y, 1.0)
}