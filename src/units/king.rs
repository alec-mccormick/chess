use crate::core::unit::{Action, UnitCmd, UnitStore, Unit, Team, Health, Actions, ActionResult, UnitComponents};
use crate::prelude::*;
use bevy::prelude::*;

use std::vec;
use std::ops::Add;

use super::utils::{list_targets_step, move_unit};


pub fn king() -> UnitComponents {
    UnitComponents {
        unit: Unit::King,
        team: Team::White,
        health: Health(1),
        position: Position::new(0, 0),
        actions: Actions(vec![Box::new(KingMoveAction)]),
    }
}


pub struct KingMoveAction;

impl Action for KingMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item=Position>> {
        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<Team>(*entity).unwrap();

        let steps: Vec<Position> = vec![
            Position::new(0, 1),
            Position::new(0, -1),
            Position::new(1, 0),
            Position::new(-1, 0),
            Position::new(1, -1),
            Position::new(1, 1),
            Position::new(-1, -1),
            Position::new(-1, 1)
        ];

        let results = steps.into_iter().filter(|step| {
            let next = position.add(*step);

            if next.x < 0 || next.y < 0 || next.x > 7 || next.y > 7 {
                return false;
            }

            if let Some(unit_entity) = store.get_unit(&next) {
                let target_team = query.get_component::<Team>(*unit_entity).unwrap();
                return target_team != team;
            }

            true
        }).collect::<Vec<Position>>();

        Box::new(results.into_iter())
    }

    fn execute(
        &self,
        entity: &Entity,
        target: &Position,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item=ActionResult>> {
        Box::new(move_unit(entity, target, store, query))
    }
}