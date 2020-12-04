mod components;
mod store;

pub use components::*;
pub use store::*;

use crate::prelude::*;
use bevy::prelude::*;
use std::ops::Deref;

use log::debug;


pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UnitCmd>()
            .add_event::<ActionResult>()
            .add_resource(UnitStore::default())
            .add_system(handle_action_result.system())
            .add_system(handle_unit_cmd_system.system())
            .add_system(UnitStore::handle_position_changed.system())
            .add_system(UnitStore::handle_health_changed.system());
    }
}

fn handle_unit_cmd_system(
    mut reader: Local<EventReader<UnitCmd>>,
    events: Res<Events<UnitCmd>>,
    mut action_events: ResMut<Events<ActionResult>>,
    store: Res<UnitStore>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
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

                for result in action.execute(&entity, &pos, &store, &action_query) {
                    action_events.send(result);
                }
            }
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
