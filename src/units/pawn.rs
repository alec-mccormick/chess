use crate::unit::{UnitAction, UnitCmd};
use crate::prelude::*;
use bevy::prelude::*;




pub struct PawnMoveAction;


impl UnitAction for PawnMoveAction {
    fn list_targets(
        entity: Entity,
        store: &Res<UnitStore>,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> dyn Iterator<Item = Position> {

        let &position = query.get_component::<&Position>(entity.clone()).unwrap();
        let &team = query.get_component::<&UnitTeam>(entity).unwrap();

        return if team.eq("White".into()) {
            list_pawn_move_targets(store, position, 1, 1).into_iter()
        } else {
            list_pawn_move_targets(store, position, -1, 6).into_iter()
        }
    }

    fn execute(
        entity: Entity,
        &target: &Position,
        store: &Res<UnitStore>,
        query: &Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth, &UnitActions)>
    ) -> dyn Iterator<Item = (ObjectId, unit::events::UnitCmd)> {

        let (&object_id, &position) = query.get(entity).unwrap();

        let commands: Vec<(ObjectId, UnitCmd)> = vec![
            (object_id, UnitCmd::SetPosition(target))
        ];

        commands.into_iter()
    }
}

fn list_pawn_move_targets(store: &Res<UnitStore>, position: &Position, step: i32, home_row: i32) -> Vec<Position> {
    let results: Vec<Position> = vec![];

    let next = position + pos(0, step);

    if store.is_position_empty(next) {
        results.push(next);

        if position.y == home_row {
            next = next + pos(0, step);

            if store.is_position_empty(next) {
                results.push(next);
            }
        }
    }

    results
}