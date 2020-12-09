mod components;
pub use components::*;

use super::{GameState, Message, PlayerType};
use crate::prelude::*;
use bevy::prelude::*;
use std::ops::Deref;

use bevy_networking::{NetworkDelivery, NetworkResource};
use log::debug;
use std::net::SocketAddr;


pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<UnitMaterials>()
            .add_event::<UnitCmd>()
            .add_event::<ActionExecuted>()
            .add_event::<ActionResult>()
            .add_position_map::<Unit>()
            .add_system(handle_action_result.system())
            .add_system(handle_unit_cmd_system.system())
            .add_system(handle_health_changed.system())
            .add_system(handle_action_executed_system.system())
            .add_system(handle_unit_spawned.system());
    }
}

// ==========================================================================
// --- Systems
// ==========================================================================
fn handle_unit_cmd_system(
    mut reader: Local<EventReader<UnitCmd>>,
    events: Res<Events<UnitCmd>>,
    mut action_events: ResMut<Events<ActionExecuted>>,
    store: Res<PositionMap<Unit>>,
    game_state: Res<GameState>,
    mut net: ResMut<NetworkResource>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    id_query: Query<(Entity, &Id)>,
) {
    for cmd in reader.iter(&events) {
        debug!("handle_unit_cmd() {:?}", cmd);

        match cmd {
            UnitCmd::ExecuteAction(entity, index, pos) => {
                let actions = action_query.get_component::<Actions>(*entity).unwrap();
                let action = actions.get(*index).unwrap();

                if !is_action_valid(action, &entity, &pos, &store, &action_query) {
                    debug!("handle_unit_cmd() - target position is invalid: {:?}", pos);
                    return;
                }

                let &id = id_query.get_component::<Id>(*entity).unwrap();

                debug!("handle_unit_cmd() - unit id: {:?}", id);

                let delivery = NetworkDelivery::ReliableSequenced(Some(1));
                let message = Message::MoveRequest(id, pos.clone()).to_bytes().unwrap();

                let remote_addr = game_state
                    .players
                    .iter()
                    .find_map(|(player_type, player_info)| -> Option<SocketAddr> {
                        if let PlayerType::Remote(addr) = player_type {
                            return Some(addr.clone());
                        }

                        None
                    })
                    .unwrap();

                net.send(remote_addr, &message, delivery).unwrap();
                action_events.send(ActionExecuted(*entity, *index, *pos));
            }
        }
    }
}


fn handle_action_executed_system(
    mut reader: Local<EventReader<ActionExecuted>>,
    events: Res<Events<ActionExecuted>>,
    mut action_events: ResMut<Events<ActionResult>>,
    store: Res<PositionMap<Unit>>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    for event in reader.iter(&events) {
        debug!("handle_action_executed() {:?}", event);

        let ActionExecuted(entity, index, pos) = event;

        let actions = action_query.get_component::<Actions>(*entity).unwrap();
        let action = actions.get(*index).unwrap();

        for result in action.execute(&entity, &pos, &store, &action_query) {
            action_events.send(result);
        }
    }
}

fn handle_action_result(
    mut reader: Local<EventReader<ActionResult>>,
    events: Res<Events<ActionResult>>,
    mut query: Query<With<Unit, (&mut Position, &mut Health)>>,
) {
    for result in reader.iter(&events) {
        debug!("handle_action_result() {:?}", result);

        match result {
            ActionResult::SetPosition(entity, position) => {
                query.set(*entity, *position).unwrap();
            }
            ActionResult::SetHealth(entity, health) => {
                query.set(*entity, *health).unwrap();
            }
        }
    }
}

fn handle_health_changed(
    mut commands: Commands,
    mut unit_position_map: ResMut<PositionMap<Unit>>,
    query: Query<With<Unit, (Entity, Mutated<Health>)>>,
) {
    for (entity, health) in query.iter() {
        debug!("handle_health_changed() {:?} {:?}", entity, *health);

        if health.0 == 0 {
            debug!("!!!Unit reduced to 0 health: {:?}", entity);
            commands.despawn(entity);
            unit_position_map.remove_entity(&entity);
        }
    }
}


fn handle_unit_spawned(
    mut commands: Commands,
    materials: Res<UnitMaterials>,
    query: Query<(Entity, Added<Unit>, &Team)>,
) {
    for (entity, unit, team) in query.iter() {
        let material = materials.get_unit_material(&*unit, team);

        commands
            .insert(entity, SpriteComponents { material, ..Default::default() })
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