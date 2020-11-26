pub mod unit;
pub mod map;

use bevy::prelude::*;
use crate::prelude::*;

use unit::{UnitPlugin, Team, UnitCmd};
use map::{MapPlugin};




#[derive(Debug)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GameState::default())
            .add_plugin(MapPlugin)
            .add_plugin(UnitPlugin)
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
    mut reader: Local<EventReader<UnitCmd>>,
    events: Res<Events<UnitCmd>>,
    mut state: ResMut<GameState>,
) {
    for cmd in reader.iter(&events) {
        if let UnitCmd::ExecuteAction(_entity, _index, _pos) = cmd {
            state.active_team = match state.active_team {
                Team::White => Team::Black,
                Team::Black => Team::White
            }
        }
    }
}