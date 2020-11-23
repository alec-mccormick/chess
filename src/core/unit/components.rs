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
    Bishop,
    // Knight,
    Rook,
    // King,
    Queen
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Pawn => "Pawn".into(),
            Unit::Bishop => "Bishop".into(),
            Unit::Rook => "Rook".into(),
            Unit::Queen => "Queen".into(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Team {
    White,
    Black
}

impl ToString for Team {
    fn to_string(&self) -> String {
        match self {
            Team::White => "White".into(),
            Team::Black => "Black".into()
        }
    }
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
    ) -> Box<dyn Iterator<Item = ActionResult>>;
}


pub fn is_action_valid(
    action: &Box<dyn Action + Send + Sync>,
    entity: &Entity,
    target: &Position,
    store: &Res<UnitStore>,
    query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
) -> bool {
    action
        .list_targets(entity, store, query)
        .any(|p| p == *target)
}

#[derive(Debug, Copy, Clone)]
pub enum ActionResult {
    SetPosition(Entity, Position),
    SetHealth(Entity, Health)
}


/// Actual Events:
///
#[derive(Debug, Copy, Clone)]
pub enum UnitCmd {
    ExecuteAction(Entity, usize, Position)
}


