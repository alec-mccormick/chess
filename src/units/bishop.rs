use crate::{
    core::unit::{Action, ActionResult, Actions, Health, Team, Unit, UnitCmd, UnitComponents, UnitStore},
    prelude::*,
};
use bevy::prelude::*;

use std::{ops::Add, vec};

use super::utils::{list_targets_step, move_unit};


pub fn bishop() -> UnitComponents {
    UnitComponents {
        unit: Unit::Bishop,
        team: Team::White,
        health: Health(1),
        position: Position::new(0, 0),
        actions: Actions(vec![Box::new(BishopMoveAction)]),
        id: Id::from_uuid(uuid::Uuid::nil()),
    }
}


pub struct BishopMoveAction;

impl Action for BishopMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    ) -> Box<dyn Iterator<Item = Position>> {
        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<Team>(*entity).unwrap();

        let steps: Vec<Position> = vec![
            Position::new(1, -1),
            Position::new(1, 1),
            Position::new(-1, -1),
            Position::new(-1, 1),
        ];

        let results = steps
            .into_iter()
            .flat_map(|step| list_targets_step(position, team, step, store, query))
            .collect::<Vec<Position>>();

        Box::new(results.into_iter())
    }

    fn execute(
        &self,
        entity: &Entity,
        target: &Position,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    ) -> Box<dyn Iterator<Item = ActionResult>> {
        Box::new(move_unit(entity, target, store, query))
    }
}
