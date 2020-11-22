mod components;
mod store;

pub use components::*;
pub use store::*;

use bevy::prelude::*;
use crate::prelude::*;



pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(UnitStorePlugin)
            .add_event::<(Entity, UnitCmd)>()
            .add_event::<ExecuteActionEvent>()
            .add_system(handle_unit_cmd_system.system())
            .add_system(handle_execute_action.system())
        ;
    }
}



fn handle_unit_cmd_system(
    mut reader: Local<EventReader<(Entity, UnitCmd)>>,
    events: Res<Events<(Entity, UnitCmd)>>,
    mut action_events: ResMut<Events<ExecuteActionEvent>>,
    mut query: Query<(&mut Position, &mut Health)>
) {
    for (entity, cmd) in reader.iter(&events) {

        match cmd {
            UnitCmd::SetPosition(pos) => {
                query.set(*entity, *pos).unwrap();
            },
            UnitCmd::SetHealth(health) => {
                query.set(*entity, *health).unwrap();
            },
            UnitCmd::ExecuteAction(index, pos) => {
                action_events.send(ExecuteActionEvent(*entity, *index, *pos));
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ExecuteActionEvent(Entity, usize, Position);

fn handle_execute_action(
    mut reader: Local<EventReader<ExecuteActionEvent>>,
    events: Res<Events<ExecuteActionEvent>>,
    mut cmd_events: ResMut<Events<(Entity, UnitCmd)>>,
    store: Res<UnitStore>,
    action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) {
    for event in reader.iter(&events) {
        let ExecuteActionEvent(entity, index, pos) = event;

        let actions = action_query.get_component::<Actions>(*entity).unwrap();
        let action = actions.get(*index).unwrap();

        for (e, cmd) in action.execute(&entity, &pos, &store, &action_query) {
            cmd_events.send((e, cmd));
        }
    }
}