use crate::{
    core::{unit::Team, GameStartedEvent, GameState},
    prelude::*,
};
use bevy::prelude::*;
use log::debug;

pub struct InfoPanelPlugin;
impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_startup_system(setup.system())
            .add_system(handle_game_started_event.system())
            .add_system(ActivePlayerView::handle_game_state_changed.system());
    }
}

fn handle_game_started_event(
    mut commands: Commands,
    mut reader: Local<EventReader<GameStartedEvent>>,
    events: Res<Events<GameStartedEvent>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _event in reader.iter(&events) {
        debug!("handle_create_game_event()");

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");

        commands
            .spawn(InfoPanelView::bundle(materials.add(Color::NONE.into())))
            .with(InfoPanelView);

        commands.with_children(|children| {
            children
                .spawn(TextComponents {
                    text: text("Active Player:".into(), font.clone()),
                    ..Default::default()
                })
                .with(ActivePlayerView);
        });
    }
}


struct InfoPanelView;
impl InfoPanelView {
    fn bundle(material: Handle<ColorMaterial>) -> NodeComponents {
        NodeComponents {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                margin: Rect {
                    left: Val::Px(0.0),
                    right: Val::Auto,
                    top: Val::Px(0.0),
                    bottom: Val::Auto,
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        }
    }
}


struct ActivePlayerView;
impl ActivePlayerView {
    fn handle_game_state_changed(state: ChangedRes<GameState>, mut query: Query<With<ActivePlayerView, &mut Text>>) {
        println!("Handle game state changed!!!");

        for mut text in query.iter_mut() {
            let team = state.active_team.to_string();
            (*text).value = format!("Active Team: [{}]", team);
        }
    }
}


// ==============================================================================
// --- Helpers
// ==============================================================================
fn text(value: String, font: Handle<Font>) -> Text {
    Text {
        value,
        font,
        style: TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..Default::default()
        },
    }
}
