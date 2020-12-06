mod components;
mod store;

pub use components::*;
pub use store::*;

use super::{Message, GameState, PlayerType};
use crate::prelude::*;
use bevy::prelude::*;
use std::ops::Deref;

use log::debug;
use bevy_networking::{NetworkDelivery, NetworkResource};
use std::net::SocketAddr;


pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UnitCmd>()
            .add_event::<ActionExecuted>()
            .add_event::<ActionResult>()
            .add_resource(UnitStore::default())
            .add_system(handle_action_result.system())
            .add_system(handle_unit_cmd_system.system())
            .add_system(UnitStore::handle_position_changed.system())
            .add_system(UnitStore::handle_health_changed.system())
            .add_system( handle_action_executed_system.system());
    }
}

fn handle_unit_cmd_system(
    mut reader: Local<EventReader<UnitCmd>>,
    events: Res<Events<UnitCmd>>,
    mut action_events: ResMut<Events<ActionExecuted>>,
    store: Res<UnitStore>,
    game_state: Res<GameState>,
    mut net: ResMut<NetworkResource>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    labels_query: Query<(Entity, &Labels)>,
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

                let labels = labels_query.get_component::<Labels>(entity.clone()).unwrap();

                let id = labels.iter().find_map(|label| {
                    let label = String::from(label);
                    if label.starts_with("id:") { Some(label) } else { None }
                }).unwrap();

                debug!("handle_unit_cmd() - unit id: {}", id);

                let delivery = NetworkDelivery::ReliableSequenced(Some(1));
                let message = Message::MoveRequest(id, pos.clone()).to_bytes().unwrap();

                let remote_addr = game_state.players.iter().find_map(|(player_type, player_info)| -> Option<SocketAddr> {
                    if let PlayerType::Remote(addr) = player_type {
                        return Some(addr.clone());
                    }

                    None
                }).unwrap();

                net.send(remote_addr, &message, delivery);
                action_events.send(ActionExecuted(*entity, *index, *pos));
            }
        }
    }
}


fn handle_action_executed_system(
    mut reader: Local<EventReader<ActionExecuted>>,
    events: Res<Events<ActionExecuted>>,
    mut action_events: ResMut<Events<ActionResult>>,
    store: Res<UnitStore>,
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
