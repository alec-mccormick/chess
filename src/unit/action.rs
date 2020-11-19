use std::collections::{HashMap};
use bevy::{prelude::*};
use derive_more::{From, Into, Deref};

use crate::prelude::*;
use super::types::*;

type ActionError = String;  // TODO


#[derive(Debug, Clone, From, Into, Deref)]
pub struct UnitActions(Vec<dyn UnitAction>);


impl UnitActions {

    pub fn list_targets(&self,
        index: u16,
        entity: Entity,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> Result<impl Iterator<Item = Position>, ActionError> {
        let action: impl UnitAction = self.0.get(index).unwrap();
        Ok(action.list_targets(entity, query))
    }

    pub fn execute(&self,
       index: u16,
       entity: Entity,
       target: Position,
       store: UnitStore,
       query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> Result<impl Iterator<Item = (ObjectId, super::events::UnitCmd)>, ActionError> {
        let action: impl UnitAction = self.0.get(index).unwrap();
        Ok(action.execute(entity, target, store, query))
    }
}


pub trait UnitAction {

    fn list_targets(
        entity: Entity,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = Position>;

    fn execute(
        entity: Entity,
        target: Position,
        store: UnitStore,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = (ObjectId, super::events::UnitCmd)>;
}

