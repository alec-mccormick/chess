use crate::core::unit::{Action, UnitCmd, UnitStore, Unit, Team, Health, Actions};
use crate::prelude::*;
use bevy::prelude::*;

use std::vec;
use std::ops::Add;




pub struct PawnMoveAction;

impl Action for PawnMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item = Position>> {

        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<Team>(*entity).unwrap();

        let (step, home_row) = match team {
            Team::White => (1, 1),
            Team::Black => (-1, 6)
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
        query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item=(Entity, UnitCmd)>> {

        let mut commands: Vec<(Entity, UnitCmd)> = vec![
            (*entity, UnitCmd::SetPosition(*target))
        ];

        if let Some(target_unit) = store.get_unit(target) {
            let team = query.get_component::<Team>(*entity).unwrap();
            let target_team = query.get_component::<Team>(*target_unit).unwrap();

            if target_team != team {
                commands.push((*target_unit, UnitCmd::SetHealth(Health(0))));
            }
        }

        Box::new(commands.into_iter())
    }
}