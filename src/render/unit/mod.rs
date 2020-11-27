use bevy::{prelude::*, ecs::Command};
use crate::prelude::*;

use crate::core::unit::{Unit, Team};

use super::utils;

/// ==========================================================================
/// Plugin
/// ==========================================================================
pub struct RenderUnitPlugin;

impl Plugin for RenderUnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<UnitMaterials>()
            .add_system(handle_unit_spawned.system())
        ;
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
        commands.add_command(RenderUnitCmd { entity, material });
    }
}

#[derive(Clone)]
struct RenderUnitCmd {
    entity: Entity,
    material: Handle<ColorMaterial>,
}

impl Command for RenderUnitCmd {
    fn write(self: Box<Self>, world: &mut World, _: &mut Resources) {
        self.insert_mesh(world);
        self.spawn_sprite(world);
    }
}

impl RenderUnitCmd {
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
            transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
            ..Default::default()
        });

        world
            .insert_one(sprite_entity, Parent(self.entity))
            .unwrap();
    }

    fn generate_transform(position: &Position) -> Transform {
        // todo: merge with the function in map rendering
        let z = 2.0 + (8 + position.x - position.y) as f32 / 16.0;
        let translation = utils::convert_position_to_vec2(position).extend(z);

        Transform::from_translation(translation)
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