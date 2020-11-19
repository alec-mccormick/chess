use std::collections::{HashMap};
use bevy::{prelude::*};
use derive_more::{From, Into, Deref};

use crate::prelude::*;



/// Components
///
#[derive(Debug, Copy, Clone)]
pub enum UnitType {
    Pawn,
}


#[derive(Debug, Copy, Clone, From, Into, Deref)]
pub struct UnitHealth(pub u32);

#[derive(Debug, Clone, From, Into, Deref)]
pub struct UnitTeam(pub String);




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
}



