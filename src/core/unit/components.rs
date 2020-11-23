use derive_more::{From, Into, Deref};
use bevy::prelude::*;
use crate::prelude::*;

use super::store::{UnitStore};



#[derive(Bundle)]
pub struct UnitComponents {
    pub unit: Unit,
    pub team: Team,
    pub health: Health,
    pub position: Position,
    pub actions: Actions,
}



// ==============================================================================
// --- Components
// ==============================================================================
#[derive(Debug, Copy, Clone)]
pub enum Unit {
    Pawn,
    // Bishop,
    // Knight,
    // Rook,
    // King,
    // Queen
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Team {
    White,
    Black
}

#[derive(Debug, Copy, Clone, From, Into, Deref)]
pub struct Health(pub u32);



type ActionError = String;  // TODO
#[derive(From, Into, Deref)]
pub struct Actions(pub Vec<Box<dyn Action + Send + Sync>>);
impl Actions {

    pub fn get(&self, index: usize) -> Result<&Box<dyn Action + Send + Sync>, ActionError> {
        self.0.get(index).ok_or(String::from("Error"))
    }
}

pub trait Action {

    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item = Position>>;

    fn execute(
        &self,
        entity: &Entity,
        target: &Position,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item = (Entity, UnitCmd)>>;
}


/// Actual Events:
///
#[derive(Debug, Copy, Clone)]
pub enum UnitCmd {
    SetPosition(Position),
    SetHealth(Health),
    ExecuteAction(usize, Position)
}


