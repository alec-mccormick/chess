use std::collections::{HashMap};
use bevy::{prelude::*};
use derive_more::{From, Into, Deref};

use crate::prelude::*;
use super::store::UnitStore;
use super::components::{UnitType, UnitTeam, UnitHealth};


type ActionError = String;  // TODO

#[derive(Debug, Clone, From, Into, Deref)]
pub struct UnitActions(Vec<dyn UnitAction>);


#[derive(Debug, Clone)]
pub struct UnitActionExecuted(u16, Position);


impl UnitActions {

    pub fn get(&self, index: u16) -> Result<impl UnitAction, ActionError> {
        self.0.get(index).ok_or(String::from("Error"))
    }
}


pub trait UnitAction {
    
    fn list_targets(
        entity: Entity,
        store: &UnitStore,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> dyn Iterator<Item = Position>;

    fn execute(
        entity: Entity,
        target: &Position,
        store: &Res<UnitStore>,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> dyn Iterator<Item = (ObjectId, super::cmd::UnitCmd)>;
}



pub struct UnitActionPlugin;

impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(EventPlugin::<UnitActionExecuted>::new())
            .add_system(UnitActionPlugin::handle_action_executed.system())
        ;
    }
}


impl UnitActionPlugin {

    fn handle_action_executed(
        mut reader: Local<EventReader<(Entity, UnitActionExecuted)>>,
        events: Res<Events<(Entity, UnitActionExecuted)>>,
        store: Res<UnitStore>,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>,
        mut cmd_events: ResMut<Events<(ObjectId, super::cmd::UnitCmd)>>,
    ) {
        for (entity, action) in reader.iter(&events) {
            let (index, pos) = (action.0, action.1);

            let &actions = query.get_component::<UnitActions>(entity).unwrap();

            let action = actions.get(index).unwrap();

            for event in action.execute(entity, &pos, &store, &query) {
                cmd_events.send(event);
            }
        }
    }
}