use crate::{
    core::unit::{Action, ActionResult, Actions, Health, Team, Unit, UnitCmd, UnitComponents, UnitStore},
    prelude::*,
};
use bevy::prelude::*;

use std::{ops::Add, vec};

use super::utils::{list_targets_step, move_unit};


pub fn knight() -> UnitComponents {
    UnitComponents {
        unit: Unit::Knight,
        team: Team::White,
        health: Health(1),
        position: Position::new(0, 0),
        actions: Actions(vec![Box::new(KnightMoveAction)]),
        labels: Labels::default(),
    }
}


pub struct KnightMoveAction;

impl Action for KnightMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    ) -> Box<dyn Iterator<Item = Position>> {
        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<Team>(*entity).unwrap();

        let steps: Vec<Position> = vec![
            Position::new(-2, 1),
            Position::new(-1, 2),
            Position::new(1, 2),
            Position::new(2, 1),
            Position::new(2, -1),
            Position::new(1, -2),
            Position::new(-1, -2),
            Position::new(-2, -1),
        ];

        let results = steps
            .into_iter()
            .filter(|step| {
                let next = position.add(*step);

                if next.x < 0 || next.y < 0 || next.x > 7 || next.y > 7 {
                    return false;
                }

                if let Some(unit_entity) = store.get_unit(&next) {
                    let target_team = query.get_component::<Team>(*unit_entity).unwrap();
                    return target_team != team;
                }

                true
            })
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
