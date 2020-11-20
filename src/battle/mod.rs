use bevy::{prelude::*, app::Events};

use rand::{self, Rng};
use std::collections::HashMap;

use crate::prelude::*;

use crate::unit::{self, UnitType, UnitHealth, UnitStore, UnitTeam, UnitPlugin};


#[derive(Debug, Default)]
pub struct BattlePlugin {

}

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {

        app
            .add_plugin(EntityPlugin)
            .add_plugin(UnitPlugin)
            .add_startup_system(init_units.system())
            .add_system(print_positions.system())
            .add_system(fire_event.system());
    }
}

fn init_units(mut commands: Commands, mut events: ResMut<Events<entity::events::EntitySpawned>>) {
    let mut _rng = rand::thread_rng();

    // let id = rng.gen::<usize>();
    let id: u32 = 596260031;
    println!("GENERATED ID: {}", id);

    let object_id = ObjectId(id);

    commands
        .spawn((
            UnitType::Pawn,
            object_id.clone(),
            Position::new(0, 0),
            UnitTeam("White".into()),
            UnitHealth(1),
        ));

    let entity = commands.current_entity().unwrap();
    events.send(entity::events::EntitySpawned(object_id, entity));
}




// fn handle_pawn_move(
//     entity: Entity,
//     target: Position,
//     query: Query<(&ObjectId, &Position, &UnitType, &UnitTeam, &UnitHealth)>
// ) -> impl Iterator<Item = (ObjectId, unit::events::UnitCmd)> {
//
//     let (&unit_id, &position) = query.get(entity).unwrap();
//
//     let results: Vec<(ObjectId, unit::events::UnitCmd)> = vec![
//         (unit_id, unit::events::UnitCmd::SetPosition(target))
//     ];
//
//     results.into_iter()
// }
//







// ===========================================================================================
// --- Helpers
// ===========================================================================================
fn print_positions(
    query: Query<(&ObjectId, &Position, &UnitType)>
) {
    for (unit_type, unit_id, pos) in query.iter() {
        println!("Unit: {:?} {:?} {:?}", unit_type, unit_id, pos);
    }
}

struct GreetTimer(Timer);

impl Default for GreetTimer {
    fn default() -> Self {
        GreetTimer(Timer::from_seconds(2.0, true))
    }
}

fn fire_event(
    time: Res<Time>,
    mut timer: Local<GreetTimer>,
    mut unit_events: ResMut<Events<(ObjectId, unit::cmd::PositionChanged)>>
) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        println!("Firing Event!");

        let id: u32 = 596260031;
        let id = ObjectId(id);

        unit_events.send((id, unit::cmd::PositionChanged(Position::new(0, 2))));
    }
}

// ===========================================================================================
// --- Tests
// ===========================================================================================
#[cfg(test)]
mod tests {
    use super::Position;

    // #[test]
    // fn basic() {
    //     let result = Position::add_mixed_int(32, 16);
    //     assert_eq!(48, result);
    // }
}