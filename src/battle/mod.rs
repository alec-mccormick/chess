pub mod unit;
pub mod graphics;

use bevy::{prelude::*, app::Events};
use rand::{self, Rng};
use std::collections::HashMap;

use crate::prelude::*;

use unit::{UnitType, UnitTeam, UnitPlugins};


#[derive(Debug, Default)]
pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {

        app.add_plugin(EntityPlugin)
            .add_plugins(UnitPlugins)
            .add_plugin(graphics::GraphicsPlugin)
            .add_startup_system(init_units.system())
            // .add_startup_system(setup_sprites.system())
            // .add_system(print_positions.system())
            .add_system(fire_event.system())
        ;
    }
}


// fn setup_sprites(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let texture_handle = asset_server.load("chess-pieces.png");
//     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 6, 2);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//
//     commands
//         .spawn(Camera2dComponents::default())
//         .spawn(SpriteSheetComponents {
//             texture_atlas: texture_atlas_handle,
//             // transform: Transform::from_scale(Vec3::splat(1.0)),
//             ..Default::default()
//         })
//         .with(Timer::from_seconds(0.1, true));
// }




fn init_units(mut commands: Commands, mut events: ResMut<Events<entity::events::EntitySpawned>>) {
    let mut _rng = rand::thread_rng();

    // let id = rng.gen::<usize>();
    let id: u32 = 596260031;
    println!("GENERATED ID: {}", id);

    let object_id = ObjectId(id);

    let white = UnitTeam("White".into());
    // let black = UnitTeam("black".into());

    crate::units::pawn::spawn(&mut commands, object_id, Position::new(0, 0), white.clone());

    let entity = commands.current_entity().unwrap();
    events.send(entity::events::EntitySpawned(object_id, entity));
}






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
    // mut unit_events: ResMut<Events<(ObjectId, battle.unit::components::events::PositionChanged)>>,
    mut events: ResMut<Events<(ObjectId, unit::UnitCmd)>>
) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        println!("Firing Event!");

        let id: u32 = 596260031;
        let id = ObjectId(id);

        let event = unit::UnitCmd::ExecuteAction(0, Position::new(0, 2));
        events.send((id, event));
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