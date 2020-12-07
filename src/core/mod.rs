use bevy::prelude::*;
use bevy_networking::{events::MessageReceived, NetworkDelivery, NetworkResource, NetworkingPlugin};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


use crate::{prelude::*, units::*};

mod game;
pub mod map;
pub mod unit;

pub use map::{Map, MapComponents, Tile, TileComponents};
pub use unit::{Action, ActionExecuted, Actions, Health, Team, Unit, UnitCmd, UnitComponents};

use game::GameDescriptor;
use unit::UnitPlugin;


/// ==========================================================================
/// Plugin
/// ==========================================================================
#[derive(Debug)]
pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_entity_map::<Id>()
            // .add_startup_system(init_networking.system())
            .add_position_map::<Tile>()
            .add_event::<CreateGameEvent>()
            .add_event::<JoinGameEvent>()
            .add_event::<GameStartedEvent>()
            .add_plugin(UnitPlugin)
            .add_system_to_stage(bevy::scene::SCENE_STAGE, Game::handle_create_game_event.system())
            .add_system_to_stage(bevy::scene::SCENE_STAGE, Game::handle_join_game_event.system())
            .add_system_to_stage(bevy::scene::SCENE_STAGE, Game::handle_network_events.system())
            .add_resource(GameState::default())
            .add_system(GameState::handle_unit_cmd.system());
    }
}

// fn init_networking(server_bind_addr: Res<ServerBindAddr>, mut net: ResMut<NetworkResource>) {
//     info!("Binding to address: {:?}", server_bind_addr.0);
//     net.bind(server_bind_addr.0).unwrap();
// }

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
pub struct GameStartedEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub team: Team,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerType {
    Local,
    Remote(SocketAddr),
}


struct Game;
impl Game {
    fn handle_create_game_event(
        mut reader: Local<EventReader<CreateGameEvent>>,
        events: Res<Events<CreateGameEvent>>,
        mut game_started_events: ResMut<Events<GameStartedEvent>>,
        mut state: ResMut<GameState>,
    ) {
        for event in reader.iter(&events) {
            debug!("handle_create_game_event() - create game: {:?}", event);

            let player_info = event.player_info.clone();
            state.init_local_player(player_info);

            game_started_events.send(GameStartedEvent);
        }
    }

    fn handle_join_game_event(
        mut reader: Local<EventReader<JoinGameEvent>>,
        events: Res<Events<JoinGameEvent>>,
        mut game_started_events: ResMut<Events<GameStartedEvent>>,
        mut state: ResMut<GameState>,
        net: Res<NetworkResource>,
    ) {
        for event in reader.iter(&events) {
            debug!("handle_join_game_event() - joining game: {:?}", event);

            let player_info = event.player_info.clone();

            state.init_local_player(player_info.clone());
            state.connection_info = ConnectionInfo::Client;

            let delivery = NetworkDelivery::ReliableSequenced(Some(1));
            let message = Message::JoinRequest(player_info).to_bytes().unwrap();
            net.send(event.server_addr, &message, delivery).unwrap();

            game_started_events.send(GameStartedEvent);
        }
    }

    fn handle_network_events(
        mut commands: Commands,
        mut reader: Local<EventReader<MessageReceived>>,
        events: Res<Events<MessageReceived>>,
        mut state: ResMut<GameState>,
        mut net: ResMut<NetworkResource>,
        mut action_executed_events: ResMut<Events<ActionExecuted>>,
        entity_id_map: Res<EntityMap<Id>>,
    ) {
        for event in reader.iter(&events) {
            let MessageReceived(conn, data) = event;

            println!("Connection! {:?}", conn);

            let from = conn.addr;
            let message = Message::from_bytes(&*data).unwrap();

            match message {
                Message::JoinRequest(player_info) => {
                    Self::handle_join_request(&mut commands, &mut state, &mut net, from, player_info);
                }
                Message::JoinResponse(player_info, game_descriptor) => {
                    Self::handle_join_response(&mut commands, &mut state, from, player_info, game_descriptor);
                }
                Message::MoveRequest(id, position) => {
                    println!("RECEIVED MOVE REQUEST: {:?} {:?}", id, position);

                    let entity = entity_id_map.get(&id).unwrap();

                    println!("Entity!: {:?}", entity);
                    action_executed_events.send(ActionExecuted(entity.clone(), 0, position));
                }
            }
        }
    }

    fn handle_join_request(
        commands: &mut Commands,
        state: &mut ResMut<GameState>,
        net: &mut ResMut<NetworkResource>,
        from: SocketAddr,
        player_info: PlayerInfo,
    ) {
        info!("handle_join_request()");

        let game_descriptor = GameDescriptor::default();

        // Add remote player to player list
        state.players.push((PlayerType::Remote(from), player_info));

        let local_player_info = state.local_player_info.clone();

        // Send response with local player info & game descriptor
        let delivery = NetworkDelivery::ReliableSequenced(Some(1));
        let message = Message::JoinResponse(local_player_info, game_descriptor.clone())
            .to_bytes()
            .unwrap();

        net.send(from, &message, delivery).unwrap();

        info!("handle_join_request() - join response sent");
        game_descriptor.spawn(commands);
    }

    fn handle_join_response(
        commands: &mut Commands,
        state: &mut ResMut<GameState>,
        from: SocketAddr,
        player_info: PlayerInfo,
        game_descriptor: GameDescriptor,
    ) {
        info!("handle_join_response()");

        // Add remote player to list
        state.players.push((PlayerType::Remote(from), player_info));

        // Spawn game & move starting state to complete
        game_descriptor.spawn(commands);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    JoinRequest(PlayerInfo),
    JoinResponse(PlayerInfo, GameDescriptor),
    MoveRequest(Id, Position),
}

impl Message {
    fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    fn from_bytes(data: &[u8]) -> bincode::Result<Self> {
        bincode::deserialize(data)
    }
}


/// ==========================================================================
/// Game State
/// ==========================================================================
#[derive(Debug, Clone)]
pub struct GameState {
    pub local_player_info: PlayerInfo,
    pub players: Vec<(PlayerType, PlayerInfo)>,
    pub active_team: Team,
    pub connection_info: ConnectionInfo,
    pub game_type: GameType,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            local_player_info: PlayerInfo {
                name: "Player 1".into(),
                team: Team::White,
            },
            players: vec![],
            active_team: Team::White,
            connection_info: ConnectionInfo::Server,
            game_type: GameType::Networked,
        }
    }
}

impl GameState {
    fn init_local_player(&mut self, player_info: PlayerInfo) {
        self.local_player_info = player_info.clone();
        self.players = vec![(PlayerType::Local, player_info)];
    }
}


impl GameState {
    fn handle_unit_cmd(
        mut reader: Local<EventReader<ActionExecuted>>,
        events: Res<Events<ActionExecuted>>,
        mut state: ResMut<GameState>,
    ) {
        for _event in reader.iter(&events) {
            state.active_team = match state.active_team {
                Team::White => Team::Black,
                Team::Black => Team::White,
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

#[derive(Debug)]
pub struct AppConfig {
    pub port: String,

    pub remote_addr: Option<String>,
}

/// ==========================================================================
/// Other
/// ==========================================================================
#[derive(Debug, Clone)]
pub enum ConnectionInfo {
    Server,
    Client,
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