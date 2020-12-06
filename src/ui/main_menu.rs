use bevy::prelude::*;
use log::{debug, info};

use crate::{
    core::{CreateGameEvent, PlayerInfo, Team},
    prelude::*,
};
use crate::core::JoinGameEvent;


pub struct CreateMainMenuEvent;

// ==========================================================================
// Components
// ==========================================================================
pub struct MainMenu;

pub struct MainMenuButton;
pub struct StartButton;
pub struct JoinButton;


// ==========================================================================
// MainMenu Bundle Spawner
// ==========================================================================
struct MainMenuSpawner {
    start_button: MainMenuButtonSpawner,
    join_button: MainMenuButtonSpawner,
}

impl EntitySpawner for MainMenuSpawner {
    fn spawn(self, commands: &mut Commands) -> &mut Commands {
        commands
            .spawn(Self::node_components())
            .with(MainMenu)
            .with_children(|commands| {
                self.start_button.spawn_as_child(commands).with(StartButton);

                self.join_button.spawn_as_child(commands).with(JoinButton);
            })
    }
}

impl MainMenuSpawner {
    fn new(materials: &Res<MainMenuMaterials>) -> Self {
        Self {
            start_button: MainMenuButtonSpawner::from_materials(materials, "Start"),
            join_button: MainMenuButtonSpawner::from_materials(materials, "Join"),
        }
    }

    fn node_components() -> NodeComponents {
        NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}


// ==========================================================================
// MainMenuButton Bundle Spawner
// ==========================================================================
struct MainMenuButtonSpawner {
    material: Handle<ColorMaterial>,
    font: Handle<Font>,
    text: &'static str,
}

impl ChildEntitySpawner for MainMenuButtonSpawner {
    fn spawn_as_child<'a, 'b>(self, commands: &'a mut ChildBuilder<'b>) -> &'a mut ChildBuilder<'b> {
        commands
            .spawn(self.button_components())
            .with(MainMenuButton)
            .with_children(|commands| {
                commands.spawn(self.text_components());
            })
    }
}

impl MainMenuButtonSpawner {
    fn from_materials(materials: &Res<MainMenuMaterials>, text: &'static str) -> Self {
        let material = materials.normal.as_weak();
        let font = materials.font.as_weak();

        Self { material, font, text }
    }

    fn button_components(&self) -> ButtonComponents {
        ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: self.material.as_weak(),
            ..Default::default()
        }
    }

    fn text_components(&self) -> TextComponents {
        TextComponents {
            text: Text {
                value: self.text.into(),
                font: self.font.as_weak(),
                style: TextStyle {
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..Default::default()
                },
            },
            ..Default::default()
        }
    }
}


// ==========================================================================
// Systems
// ==========================================================================
pub fn handle_create_main_menu_event(
    mut commands: Commands,
    mut reader: Local<EventReader<CreateMainMenuEvent>>,
    events: Res<Events<CreateMainMenuEvent>>,
    main_menu_materials: Res<MainMenuMaterials>,
) {
    for _ in reader.iter(&events) {
        info!("handle_create_main_menu_event()");
        MainMenuSpawner::new(&main_menu_materials).spawn(&mut commands);
    }
}


/// Handles button interaction changes & updates the button material
/// to reflect its interaction state.
pub fn handle_main_menu_button_interaction(
    main_menu_materials: Res<MainMenuMaterials>,
    mut interaction_query: Query<With<MainMenuButton, (Mutated<Interaction>, &mut Handle<ColorMaterial>)>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        *material = match *interaction {
            Interaction::Clicked => main_menu_materials.pressed.clone(),
            Interaction::Hovered => main_menu_materials.hovered.clone(),
            Interaction::None => main_menu_materials.normal.clone(),
        }
    }
}

/// The system listens for when "Start" button is pressed & fires `CreateGameEvent`.
pub fn handle_start_button_pressed(
    mut commands: Commands,
    mut create_game_events: ResMut<Events<CreateGameEvent>>,
    main_menu_query: Query<With<MainMenu, Entity>>,
    interaction_query: Query<With<StartButton, Mutated<Interaction>>>,
) {
    let clicks = interaction_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Clicked)
        .next();

    if clicks.is_none() {
        return;
    }

    debug!("handle_start_button_pressed()");

    for entity in main_menu_query.iter() {
        commands.despawn_recursive(entity);
    }

    create_game_events.send(CreateGameEvent {
        player_info: PlayerInfo {
            name: "Player 1".into(),
            team: Team::White,
        },
    });
}

pub fn handle_join_button_pressed(
    mut commands: Commands,
    mut join_game_events: ResMut<Events<JoinGameEvent>>,
    main_menu_query: Query<With<MainMenu, Entity>>,
    interaction_query: Query<With<JoinButton, Mutated<Interaction>>>,
) {
    let clicks = interaction_query
        .iter()
        .filter(|interaction| **interaction == Interaction::Clicked)
        .next();

    if clicks.is_none() {
        return;
    }

    debug!("handle_join_button_pressed()");

    let server_addr = std::env::var("SERVER_ADDR").unwrap();

    for entity in main_menu_query.iter() {
        commands.despawn_recursive(entity);
    }

    join_game_events.send(JoinGameEvent {
        player_info: PlayerInfo {
            name: "Player2".into(),
            team: Team::Black,
        },
        server_addr: server_addr.parse().expect("Unable to parse socket address")
    });
}

/// ==========================================================================
/// Resources
/// ==========================================================================
pub struct MainMenuMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromResources for MainMenuMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        MainMenuMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}
