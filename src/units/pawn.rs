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

        let results = match team {
            Team::White => list_pawn_move_targets(store, position, 1, 1),
            Team::Black => list_pawn_move_targets(store, position, -1, 6),
        };

        Box::new(results.into_iter())
    }

    fn execute(
        &self,
        entity: &Entity,
        &target: &Position,
        _store: &Res<UnitStore>,
        _query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>
    ) -> Box<dyn Iterator<Item=(Entity, UnitCmd)>> {

        // let &position = query.get_component::<Position>(*entity).unwrap();

        let commands: Vec<(Entity, UnitCmd)> = vec![
            (*entity, UnitCmd::SetPosition(target))
        ];

        Box::new(commands.into_iter())
    }
}

fn list_pawn_move_targets(store: &Res<UnitStore>, position: &Position, step: i32, home_row: i32) -> Vec<Position> {
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

    results
}