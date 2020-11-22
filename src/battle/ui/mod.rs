use bevy::prelude::*;
use crate::prelude::*;

use super::map;



#[derive(Debug, Clone, Default)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(sprite_interaction_system.system())
            .add_system(button_system.system())
        ;

    }
}


#[derive(Default)]
struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
    hovered_entity: Option<Entity>,
}

fn button_system(
    mut interaction_query: Query<With<map::Tile, (Mutated<Interaction>, &Position)>>,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, position) in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Clicked => {
                println!("Pressed! {:?}", position);
            }
            Interaction::Hovered => {
                println!("Hover {:?}", position);
            }
            Interaction::None => {
                println!("None {:?}", position);
            }
        }
    }
}


fn sprite_interaction_system(
    mut state: Local<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    touches_input: Res<Touches>,
    window: Res<WindowDescriptor>,
    mut query: Query<(Entity, &Sprite, &GlobalTransform, &mut Interaction)>
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

    for (entity, sprite, global_transform, mut interaction) in query.iter_mut() {
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
    }

    if let Some(new_hovered_entity) = hovered_entity {
        if let Some(old_hovered_entity) = state.hovered_entity {
            if new_hovered_entity != old_hovered_entity {
                if let Ok(mut interaction) =
                query.get_component_mut::<Interaction>(old_hovered_entity)
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