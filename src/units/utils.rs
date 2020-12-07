use crate::prelude::*;
use bevy::prelude::*;

use crate::core::unit::{Action, ActionResult, Actions, Health, Team, Unit, UnitCmd};

use std::{ops::Add, vec};

pub fn list_targets_step(
    starting_position: &Position,
    team: &Team,
    step: Position,
    store: &Res<PositionMap<Unit>>,
    query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) -> vec::IntoIter<Position> {
    let mut results: Vec<Position> = vec![];

    let mut next_pos = Some(starting_position.add(step));

    while let Some(next) = next_pos {
        if next.x < 0 || next.y < 0 || next.x > 7 || next.y > 7 {
            next_pos = None;
        } else if let Some(unit_entity) = store.get(&next) {
            let target_team = query.get_component::<Team>(*unit_entity).unwrap();
            if target_team != team {
                results.push(next.clone());
            }

            next_pos = None;
        } else {
            results.push(next.clone());
            next_pos = Some(next.add(step));
        }
    }


    return results.into_iter();
}


pub fn move_unit(
    entity: &Entity,
    target: &Position,
    store: &Res<PositionMap<Unit>>,
    query: &Query<(&Unit, &Position, &Team, &Health, &Actions)>,
) -> vec::IntoIter<ActionResult> {
    let mut commands: Vec<ActionResult> = vec![ActionResult::SetPosition(*entity, *target)];

    if let Some(target_unit) = store.get(target) {
        let team = query.get_component::<Team>(*entity).unwrap();
        let target_team = query.get_component::<Team>(*target_unit).unwrap();

        if target_team != team {
            commands.push(ActionResult::SetHealth(*target_unit, Health(0)));
        }
    }

    commands.into_iter()
}
