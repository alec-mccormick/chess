use bevy::{prelude::*, ecs::Command};

use log::{debug};

use crate::prelude::*;
use crate::units::*;

use crate::core::{
    map::{TileComponents, Tile, MapComponents, Map},
    unit::{UnitComponents, Unit, Team, Health, Actions},
};



pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ButtonMaterials>()
            .add_startup_system(spawn_main_menu.system())
            .add_system_to_stage(stage::PRE_UPDATE, handle_start_button_pressed.system())
        ;
    }
}


/// ==========================================================================
/// Main Menu
/// ==========================================================================
fn handle_start_button_pressed(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    main_menu_query: Query<With<MainMenu, Entity>>,
    mut interaction_query: Query<With<MainMenuButton, (Mutated<Interaction>, &mut Handle<ColorMaterial>)>>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                debug!("handle_start_button_pressed()");
                *material = button_materials.pressed.clone();

                for entity in main_menu_query.iter() {
                    commands.despawn_recursive(entity);
                }

                let map = commands
                    .spawn(MapComponents::default())
                    .current_entity()
                    .unwrap();

                println!("Map Spawned");

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
                    commands.spawn(UnitComponents { team: Team::White, position: Position::new(x, 1), ..pawn() }).with(Parent(map));
                    commands.spawn(UnitComponents { team: Team::Black, position: Position::new(x, 6), ..pawn() }).with(Parent(map));
                }

                let team = Team::White;
                commands.spawn(UnitComponents { team, position: Position::new(0, 0), ..rook() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(1, 0), ..knight() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(2, 0), ..bishop() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(3, 0), ..queen() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(4, 0), ..king() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(5, 0), ..bishop() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(6, 0), ..knight() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(7, 0), ..rook() }).with(Parent(map));


                let team = Team::Black;
                commands.spawn(UnitComponents { team, position: Position::new(0, 7), ..rook() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(1, 7), ..knight() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(2, 7), ..bishop() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(3, 7), ..queen() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(4, 7), ..king() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(5, 7), ..bishop() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(6, 7), ..knight() }).with(Parent(map));
                commands.spawn(UnitComponents { team, position: Position::new(7, 7), ..rook() }).with(Parent(map));
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}



/// ==========================================================================
/// Main Menu
/// ==========================================================================
pub struct MainMenu;
pub struct MainMenuButton;

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
        // .with_bundle((MainMenuButton, Parent(main_menu_entity)))
        .with_bundle((MainMenu, MainMenuButton))
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