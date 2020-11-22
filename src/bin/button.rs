use bevy::prelude::*;
use bevy::core::FloatOrd;

use chess::prelude::*;
use bevy::ui::FocusPolicy;

/// This example illustrates how to create a button that changes color and text based on its interaction state.
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 1680,
            height: 1050,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<TileMaterials>()
        .add_startup_system(setup.system())
        .add_system(tile_ui.system())
        .add_system(button_system.system())
        .run();
}

#[derive(Debug, Clone)]
struct Tile;

struct TileMaterials {
    white: Handle<ColorMaterial>,
    black: Handle<ColorMaterial>,
}

impl FromResources for TileMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        let black_tile_handle = asset_server.load("black.png");
        let white_tile_handle = asset_server.load("white.png");

        TileMaterials {
            white: materials.add(white_tile_handle.into()),
            black: materials.add(black_tile_handle.into()),
        }
    }
}

fn button_system(
    mut interaction_query: Query<(Mutated<Interaction>)>,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction) in interaction_query.iter_mut() {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        
        match *interaction {
            Interaction::Clicked => {
                println!("Pressed!");
                // text.value = "Press".to_string();
                // *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                println!("Hover");
                // text.value = "Hover".to_string();
                // *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                println!("None");
                // text.value = "Button".to_string();
                // *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(mut commands: Commands, tile_materials: Res<TileMaterials>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
    ;

    create_button(Position::new(0, 0), &tile_materials, &mut commands);
    // create_button(Position::new(1, 0), &tile_materials, &mut commands);
}

fn create_button(position: Position, tile_materials: &Res<TileMaterials>, commands: &mut Commands){

    let material = if (position.x + position.y) % 2 == 0 {
        tile_materials.black.clone()
    } else {
        tile_materials.white.clone()
    };

    let transform = {
        Transform {
            translation: convert_position_to_translation(&position),
            rotation: Quat::identity(),
            scale: Vec3::splat(0.5),
        }
    };

    commands
        .spawn((
            Tile,
            position,
            Interaction::default(),
        ))
        .with_bundle(SpriteComponents {
            material,
            transform,
            ..Default::default()
        })
    ;
}

fn convert_position_to_translation(position: &Position) -> Vec3 {
    let x = ((position.x as f32) - 4.0) * 100.0 + 50.0;
    let y = ((position.y as f32) - 4.0) * 100.0 + 50.0;

    println!("X: {}", x);
    println!("Y: {}", y);
    Vec3::new(x, y, 0.0)
}

#[derive(Default)]
pub struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
    hovered_entity: Option<Entity>,
}

fn tile_ui(
    mut state: Local<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    touches_input: Res<Touches>,
    window: Res<WindowDescriptor>,
    mut node_query: Query<With<Tile, (
        Entity,
        &Sprite,
        &GlobalTransform,
        &mut Interaction
    )>>
) {
    let mut cursor_changed = false;

    if let Some(cursor_moved) = state.cursor_moved_event_reader.latest(&cursor_moved_events) {
        state.cursor_position = cursor_moved.position;
        cursor_changed = true;
    }
    if let Some(touch) = touches_input.get_pressed(0) {
        state.cursor_position = touch.position;
        cursor_changed = true;
    }

    let mouse_clicked = mouse_button_input.just_released(MouseButton::Left) || touches_input.just_released(0);

    if !cursor_changed && !mouse_clicked {
        return;
    }

    let mut hovered_entity = None;

    for (entity, sprite, global_transform, mut interaction) in node_query.iter_mut() {
        let position = global_transform.translation.truncate();
        let size = sprite.size * global_transform.scale.truncate();

        let x = (window.width as f32 / 2.0) + position.x() - (size.x() / 2.0);
        let y = (window.height as f32 / 2.0) + position.y() - (size.y() / 2.0);

        let diff_x = state.cursor_position.x() - x;
        let diff_y = state.cursor_position.y() - y;

        let is_hovered = (diff_x >= 0.0 && diff_x < size.x())
            && (diff_y >= 0.0 && diff_y < size.y());

        if is_hovered {
            if mouse_clicked {
                if *interaction != Interaction::Clicked {
                    *interaction = Interaction::Clicked;
                }
            } else if *interaction != Interaction::Hovered {
                *interaction = Interaction::Hovered;
            }

            hovered_entity = Some(entity);
        }

        // println!("position: {:?}, size: {:?}", (x, y), size);
    }

    if let Some(new_hovered_entity) = hovered_entity {
        if let Some(old_hovered_entity) = state.hovered_entity {
            if new_hovered_entity != old_hovered_entity {
                if let Ok(mut interaction) =
                node_query.get_component_mut::<Interaction>(old_hovered_entity)
                {
                    if *interaction == Interaction::Hovered {
                        *interaction = Interaction::None;
                    }
                }
                state.hovered_entity = None;
            }
        }
        state.hovered_entity = hovered_entity;
    }
}