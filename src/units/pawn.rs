use crate::{
    core::unit::{Action, ActionResult, Actions, Health, Team, Unit, UnitCmd, UnitComponents, UnitStore},
    prelude::*,
};
use bevy::prelude::*;

use std::{ops::Add, vec};

use super::utils::move_unit;


pub fn pawn() -> UnitComponents {
    UnitComponents {
        unit: Unit::Pawn,
        team: Team::White,
        health: Health(1),
        position: Position::new(0, 0),
        actions: Actions(vec![Box::new(PawnMoveAction)]),
    }
}


pub struct PawnMoveAction;

impl Action for PawnMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
    ) -> Box<dyn Iterator<Item = Position>> {
        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<Team>(*entity).unwrap();

        let (step, home_row) = match team {
            Team::White => (1, 1),
            Team::Black => (-1, 6),
        };

        let mut results: Vec<Position> = vec![];

        let mut next = position.add(Position::new(0, step));

        if store.is_position_empty(&next) {
            results.push(next.clone());

            if position.y == home_row {
                next = next.add(Position::new(0, step));

                if store.is_position_empty(&next) {
                    results.push(next.clone());
                }
            }
        }

        if position.x > 0 {
            let attack_left_position = position.add(Position::new(-1, step));

            if let Some(unit) = store.get_unit(&attack_left_position) {
                let target_team = query.get_component::<Team>(*unit).unwrap();
                if target_team != team {
                    results.push(attack_left_position);
                }
            }
        }

        if position.x < 7 {
            let attack_right_position = position.add(Position::new(1, step));

            if let Some(unit) = store.get_unit(&attack_right_position) {
                let target_team = query.get_component::<Team>(*unit).unwrap();
                if target_team != team {
                    results.push(attack_right_position);
                }
            }
        }

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
