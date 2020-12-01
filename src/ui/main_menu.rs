use bevy::{prelude::*, ecs::Command};

use log::{debug};

use crate::prelude::*;
use crate::core::{Team, CreateGameEvent};



pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ButtonMaterials>()
            .add_startup_system(spawn_main_menu.system())
            .add_system(handle_main_menu_button.system())
            .add_system_to_stage(stage::UPDATE, handle_start_button.system())
        ;
    }
}


/// ==========================================================================
/// Main Menu
/// ==========================================================================
fn handle_main_menu_button(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<With<MainMenuButton, (Mutated<Interaction>, &mut Handle<ColorMaterial>)>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        *material = match *interaction {
            Interaction::Clicked => button_materials.pressed.clone(),
            Interaction::Hovered => button_materials.hovered.clone(),
            Interaction::None => button_materials.normal.clone(),
        }
    }
}

fn handle_start_button(
    mut commands: Commands,
    mut create_game_events: ResMut<Events<CreateGameEvent>>,
    main_menu_query: Query<With<MainMenu, Entity>>,
    interaction_query: Query<With<StartButton, Mutated<Interaction>>>,
) {
    let clicks = interaction_query.iter()
        .filter(|interaction| **interaction == Interaction::Clicked);

    for _ in clicks {
        for entity in main_menu_query.iter() {
            commands.despawn_recursive(entity);
        }

        debug!("handle_start_button() - Start Button Pressed");
        create_game_events.send(CreateGameEvent {
            player_name: "Alec".into(),
            team: Team::White,
        });
    }
}



/// ==========================================================================
/// Main Menu
/// ==========================================================================
pub struct MainMenu;

fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(80.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_bundle((MainMenu, MainMenuButton, StartButton))
        .with_children(|children| {
            children.spawn(TextComponents {
                text: Text {
                    value: "Start".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}

pub struct StartButton;
pub struct MainMenuButton;

/// ==========================================================================
/// Resources
/// ==========================================================================
struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}