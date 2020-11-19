use std::collections::{HashMap};
use bevy::{prelude::*};
use derive_more::{From, Into, Deref};

use crate::prelude::*;
use super::types::*;





pub trait UnitAction {

    fn list_targets(
        entity: Entity,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = Position>;

    fn execute(
        entity: Entity,
        target: Position,
        query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
    ) -> dyn Iterator<Item = (ObjectId, super::events::UnitCmd)>;
}

