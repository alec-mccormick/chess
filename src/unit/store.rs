use std::collections::{HashMap};
use bevy::prelude::*;

use crate::prelude::*;
use super::components;


/// UnitStorage
///
#[derive(Debug, Default)]
pub struct UnitStore {
    unit_id_to_position: HashMap<ObjectId, Position>,
    position_to_unit_id: HashMap<Position, ObjectId>,
}


impl UnitStore {

    pub fn set_position(&mut self, unit_id: ObjectId, position: Position) {
        if let Some(current_position) = self.unit_id_to_position.get(&unit_id) {
            match self.position_to_unit_id.get(&current_position) {
                None => {}
                Some(existing_unit_id) => {
                    if unit_id.eq(existing_unit_id) {
                        self.position_to_unit_id.remove(&current_position);
                    }
                }
            }
        }

        self.unit_id_to_position.insert(unit_id, position);
        self.position_to_unit_id.insert(position, unit_id);
    }

    pub fn remove_position(&mut self, unit_id: ObjectId) {
        if let Some(current_position) = self.unit_id_to_position.get(&unit_id) {
            match self.position_to_unit_id.get(&current_position) {
                None => {}
                Some(existing_unit_id) => {
                    if unit_id.eq(existing_unit_id) {
                        self.position_to_unit_id.remove(&current_position);
                    }
                }
            }

            self.unit_id_to_position.remove(&unit_id);
        }
    }

    pub fn is_position_empty(&self, position: &Position) -> bool {
        self.position_to_unit_id.get(position).is_none()
    }
}




#[derive(Debug, Default)]
pub struct UnitStorePlugin;

impl Plugin for UnitStorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(UnitStore::default())
            .add_system(UnitStorePlugin::handle_position_changed.system())
            .add_system(UnitStorePlugin::handle_entity_spawned.system());
    }
}


impl UnitStorePlugin {

    fn handle_position_changed(
        mut reader: Local<EventReader<(ObjectId, components::events::PositionChanged)>>,
        events: Res<Events<(ObjectId, components::events::PositionChanged)>>,
        mut store: ResMut<UnitStore>
    ) {
        for (id, event) in reader.iter(&events) {
            store.set_position(*id, event.0);
        }
    }

    fn handle_entity_spawned(
        mut spawned_reader: Local<EventReader<entity::events::EntitySpawned>>,
        spawned_events: Res<Events<entity::events::EntitySpawned>>,
        mut store: ResMut<UnitStore>,
        query: Query<(&Position)>,
    ) {
        for event in spawned_reader.iter(&spawned_events) {
            let (object_id, entity) = (event.0, event.1);

            let &position = query.get(entity).unwrap();
            store.set_position(object_id, position);
        }
    }
}
