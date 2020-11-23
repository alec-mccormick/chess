pub mod unit;
pub mod map;

use bevy::prelude::*;
use crate::prelude::*;

use unit::{UnitPlugin, Team, UnitCmd};
use map::{MapPlugin};

/// Todo:
/// x GameState, swap turns with each move (needs to be fixed to not work on invalid moves)
/// x Validate move before executing
/// - Highlight moves
/// - Show current player with text
/// - Implement attacking
///     - Handle health changes in various modules (despawn + remove sprite)



#[derive(Debug, Clone)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GameState::default())
            .add_plugin(UnitPlugin)
            .add_plugin(MapPlugin)
            .add_system(handle_unit_cmd.system())
        ;
    }
}



#[derive(Debug, Clone)]
pub struct GameState {
    pub active_team: Team,
}


impl Default for GameState {
    fn default() -> Self {
        GameState {
            active_team: Team::White
        }
    }
}


fn handle_unit_cmd(
    mut reader: Local<EventReader<(Entity, UnitCmd)>>,
    events: Res<Events<(Entity, UnitCmd)>>,
    mut state: ResMut<GameState>,
) {
    for (_entity, cmd) in reader.iter(&events) {
        if let UnitCmd::ExecuteAction(index, pos) = cmd {
            state.active_team = match state.active_team {
                Team::White => Team::Black,
                Team::Black => Team::White
            }
        }
    }
}