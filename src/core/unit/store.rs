use std::collections::{HashMap};
use bevy::prelude::*;

use crate::prelude::*;
use super::components::*;


/// UnitStorage
///
#[derive(Debug, Default)]
pub struct UnitStore {
    unit_id_to_position: HashMap<Entity, Position>,
    position_to_unit_id: HashMap<Position, Entity>,
}


impl UnitStore {

    pub fn set_position(&mut self, unit_id: Entity, position: Position) {
        self.remove_position_inner(unit_id);

        self.unit_id_to_position.insert(unit_id, position);
        self.position_to_unit_id.insert(position, unit_id);
    }

    pub fn remove(&mut self, unit_id: Entity) {
        self.remove_position_inner(unit_id);
        self.unit_id_to_position.remove(&unit_id);
    }

    fn remove_position_inner(&mut self, unit_id: Entity) -> Option<()> {
        let current_position = self.unit_id_to_position.get(&unit_id)?;
        let existing_unit_id = self.position_to_unit_id.get(&current_position)?;

        if unit_id.eq(existing_unit_id) {
            self.position_to_unit_id.remove(&current_position);
        }

        Some(())
    }

    pub fn is_position_empty(&self, position: &Position) -> bool {
        self.get_unit(position).is_none()
    }

    pub fn get_unit(&self, position: &Position) -> Option<&Entity> {
        self.position_to_unit_id.get(position)
    }
}



impl UnitStore {

    pub fn handle_position_changed(
        mut store: ResMut<UnitStore>,
        query: Query<With<Unit, (Entity, Changed<Position>)>>
    ) {
        for (entity, position) in query.iter() {
            println!("UnitStore::handle_position_changed() {:?} {:?}", entity, *position);
            store.set_position(entity, *position);
        }
    }

    pub fn handle_health_change(
        mut commands: Commands,
        mut store: ResMut<UnitStore>,
        query: Query<With<Unit, (Entity, Mutated<Health>)>>
    ) {
        for (entity, health) in query.iter() {
            if health.0 == 0 {
                println!("!!!Unit reduced to 0 health: {:?}", entity);
                commands.despawn(entity);
                store.remove(entity);
            }
        }
    }
}
