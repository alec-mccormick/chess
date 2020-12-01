pub mod unit;
pub mod map;

use unit::{UnitPlugin};
use map::{MapPlugin};

pub use map::{Map, MapComponents, Tile, TileComponents};
pub use unit::{UnitComponents, Unit, Team, UnitCmd, Health, Action, Actions};

use bevy::prelude::*;
use log::{debug};

use crate::prelude::*;
use crate::units::*;



/// ==========================================================================
/// Plugin
/// ==========================================================================
#[derive(Debug)]
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<CreateGameEvent>()
            .add_plugin(MapPlugin)
            .add_plugin(UnitPlugin)
            .add_system_to_stage(bevy::scene::SCENE_STAGE, Game::handle_create_game_event.system())
            .add_resource(GameState::default())
            .add_system(GameState::handle_unit_cmd.system())
        ;
    }
}

/// ==========================================================================
/// Game
/// ==========================================================================
#[derive(Debug, Clone)]
pub struct CreateGameEvent {
    pub player_name: String,
    pub team: Team
}

struct Game;
impl Game {
    fn handle_create_game_event(
        mut commands: Commands,
        mut reader: Local<EventReader<CreateGameEvent>>,
        events: Res<Events<CreateGameEvent>>,
    ) {
        for event in reader.iter(&events) {
            debug!("handle_create_game_event() - create game: {:?}", event);

            let map = commands
                .spawn(MapComponents::default())
                .current_entity()
                .unwrap();

            debug!("handle_create_game_event() - Map Spawned");

            for x in 0..=7 {
                for y in 0..=7 {
                    let position = Position::new(x, y);
                    let tile = if (x + y) % 2 == 0 { Tile::Black } else { Tile::White };

                    commands
                        .spawn(TileComponents { tile, position })
                        .with(Parent(map));
                }
            }

            for x in 0..=7 {
                commands.spawn(UnitComponents { team: Team::White, position: (x, 1).into(), ..pawn() }).with(Parent(map));
                commands.spawn(UnitComponents { team: Team::Black, position: (x, 6).into(), ..pawn() }).with(Parent(map));
            }

            let team = Team::White;
            commands.spawn(UnitComponents { team, position: (0, 0).into(), ..rook() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (1, 0).into(), ..knight() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (2, 0).into(), ..bishop() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (3, 0).into(), ..queen() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (4, 0).into(), ..king() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (5, 0).into(), ..bishop() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (6, 0).into(), ..knight() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (7, 0).into(), ..rook() }).with(Parent(map));


            let team = Team::Black;
            commands.spawn(UnitComponents { team, position: (0, 7).into(), ..rook() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (1, 7).into(), ..knight() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (2, 7).into(), ..bishop() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (3, 7).into(), ..queen() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (4, 7).into(), ..king() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (5, 7).into(), ..bishop() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (6, 7).into(), ..knight() }).with(Parent(map));
            commands.spawn(UnitComponents { team, position: (7, 7).into(), ..rook() }).with(Parent(map));

            debug!("handle_create_game_event() - Units Spawned");
        }
    }
}


/// ==========================================================================
/// Game State
/// ==========================================================================
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

impl GameState {
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
}