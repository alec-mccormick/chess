use crate::battle::unit::{UnitAction, UnitCmd, UnitStore, UnitType, UnitTeam, UnitHealth, UnitActions};
use crate::prelude::*;
use bevy::prelude::*;

use std::vec;
use std::ops::Add;


pub fn spawn(commands: &mut Commands, object_id: ObjectId, position: Position, team: UnitTeam) {
    commands.spawn((
        UnitType::Pawn,
        object_id.clone(),
        position,
        team,
        UnitHealth(1),
        UnitActions(vec![Box::new(PawnMoveAction)])
    ));
}



pub struct PawnMoveAction;

impl UnitAction for PawnMoveAction {
    fn list_targets(
        &self,
        entity: &Entity,
        store: &Res<UnitStore>,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> Box<dyn Iterator<Item = Position>> {

        let position = query.get_component::<Position>(*entity).unwrap();
        let team = query.get_component::<UnitTeam>(*entity).unwrap();

        let results = if team.eq("White".into()) {
            list_pawn_move_targets(store, position, 1, 1)
        } else {
            list_pawn_move_targets(store, position, -1, 6)
        };

        Box::new(results.into_iter())
    }

    fn execute(
        &self,
        entity: &Entity,
        &target: &Position,
        _store: &Res<UnitStore>,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> Box<dyn Iterator<Item=(ObjectId, UnitCmd)>> {

        let &object_id = query.get_component::<ObjectId>(*entity).unwrap();
        let &position = query.get_component::<Position>(*entity).unwrap();

        let commands: Vec<(ObjectId, UnitCmd)> = vec![
            (object_id, UnitCmd::SetPosition(target))
        ];

        Box::new(commands.into_iter())
    }
}

fn list_pawn_move_targets(store: &Res<UnitStore>, position: &Position, step: i32, home_row: i32) -> Vec<Position> {
    let mut results: Vec<Position> = vec![];

    let mut next = position.add(pos(0, step));

    if store.is_position_empty(&next) {
        results.push(next.clone());

        if position.y == home_row {
            next = next.add(pos(0, step));

            if store.is_position_empty(&next) {
                results.push(next.clone());
            }
        }
    }

    results
}