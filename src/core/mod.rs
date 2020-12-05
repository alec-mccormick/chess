use bevy::prelude::*;
use bevy_networking::{NetworkDelivery, NetworkResource, NetworkingPlugin, events::MessageReceived};
use log::{debug, info};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};


use crate::{prelude::*, units::*};

pub mod map;
pub mod unit;
mod game;

pub use map::{Map, MapComponents, Tile, TileComponents};
pub use unit::{Action, Actions, Health, Team, Unit, UnitCmd, UnitComponents};

use map::MapPlugin;
use unit::UnitPlugin;
use game::GameDescriptor;


const SERVER: &str = "127.0.0.1:12351";
const CLIENT: &str = "127.0.0.1:12350";

/// ==========================================================================
/// Plugin
/// ==========================================================================
#[derive(Debug)]
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(NetworkingPlugin)
            .add_startup_system(init_networking.system())
            .add_event::<CreateGameEvent>()
            .add_event::<JoinGameEvent>()
            .add_plugin(MapPlugin)
            .add_plugin(UnitPlugin)
            .add_system_to_stage(bevy::scene::SCENE_STAGE, Game::handle_create_game_event.system())
            .add_resource(GameState::default())
            .add_system(GameState::handle_unit_cmd.system());
    }
}

fn init_networking(server_bind_addr: Res<ServerBindAddr>, mut net: ResMut<NetworkResource>) {
    info!("Binding to address: {:?}", server_bind_addr.0);
    net.bind(server_bind_addr.0).unwrap();
}

/// ==========================================================================
/// Game
/// ==========================================================================
#[derive(Debug, Clone)]
pub struct CreateGameEvent {
    pub player_info: PlayerInfo,
}

#[derive(Debug, Clone)]
pub struct JoinGameEvent {
    pub player_info: PlayerInfo,
    pub server_addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub team: Team,
}


struct Game;
impl Game {
    fn handle_create_game_event(
        mut commands: Commands,
        mut reader: Local<EventReader<CreateGameEvent>>,
        events: Res<Events<CreateGameEvent>>,
        mut state: ResMut<GameState>,
        // net: Res<NetworkResource>,
    ) {
        for event in reader.iter(&events) {
            debug!("handle_create_game_event() - create game: {:?}", event);

            state.player_info = event.player_info.clone();
            GameDescriptor::default().spawn(&mut commands);
        }
    }

    fn handle_join_game_event(
        mut commands: Commands,
        mut reader: Local<EventReader<JoinGameEvent>>,
        events: Res<Events<JoinGameEvent>>,
        mut state: ResMut<GameState>,
        // net: Res<NetworkResource>,
    ) {
        for event in reader.iter(&events) {
            debug!("handle_join_game_event() - joining game: {:?}", event);

            state.connection_info = ConnectionInfo::Client(event.server_addr);
            state.player_info = event.player_info.clone();
            GameDescriptor::default().spawn(&mut commands);
        }
    }
}


pub enum GameStartingState {
    Loading,
    WaitingForClient,
    Complete,
}

/// ==========================================================================
/// Game State
/// ==========================================================================
#[derive(Debug, Clone)]
pub struct GameState {
    pub player_info: PlayerInfo,
    pub active_team: Team,
    pub connection_info: ConnectionInfo,
    pub game_type: GameType,
}


impl Default for GameState {
    fn default() -> Self {
        GameState {
            player_info: PlayerInfo {
                name: "Player 1".into(),
                team: Team::White,
            },
            active_team: Team::White,
            connection_info: ConnectionInfo::Server,
            game_type: GameType::Networked,
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
                    Team::Black => Team::White,
                }
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum GameType {
    // Hotseat,
    Local,
    Networked,
}


/// ==========================================================================
/// Other
/// ==========================================================================
#[derive(Debug, Clone)]
pub enum ConnectionInfo {
    Server,
    Client(SocketAddr),
}

impl ConnectionInfo {
    pub fn is_server(&self) -> bool {
        match &self {
            ConnectionInfo::Server => true,
            _ => false,
        }
    }

    pub fn is_client(&self) -> bool {
        !self.is_server()
    }
}

pub struct ServerBindAddr(pub SocketAddr);
