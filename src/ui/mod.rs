pub mod info_panel;
mod sprite_interaction;
mod map;
mod input;


use bevy::prelude::*;
use crate::prelude::*;

use crate::core::{
    GameState,
    map::Tile,
    unit::{UnitStore, UnitCmd, Unit, Actions, Team, Health, is_action_valid}
};


pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(input::InputState::Default)
            .add_startup_system(setup.system())
            .add_plugin(info_panel::InfoPanelPlugin)
            .add_plugin(map::MapUIPlugin)
            .add_system(sprite_interaction::sprite_interaction_system.system())
            .add_system(input::handle_tile_interaction.system())
            .add_system(map::handle_input_state_change.system())
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(UiCameraComponents::default());
}



// pub enum UIState {
//     Default,
//     UnitSelected(Entity)
// }
//
// pub struct UIStateStore {
//     state: UIState,
// }
//
// impl Default for UIStateStore {
//     fn default() -> Self {
//         UIStateStore { state: UIState::Default }
//     }
// }
//
// impl UIState {
//
//     pub fn system(
//         mut store: ResMut<UIStateStore>,
//         game_state: Res<GameState>,
//         unit_store: Res<UnitStore>,
//         mut cmds: ResMut<Events<UnitCmd>>,
//         mut interaction_query: Query<With<Tile, (Mutated<Interaction>, &Position)>>,
//         action_query: Query<(&Unit, &Position, &Team, &Health, &Actions)>,
//     ) {
//         for (interaction, position) in interaction_query.iter_mut() {
//
//             match *interaction {
//                 Interaction::Clicked => {
//                     match store.state {
//                         UIState::Default => {
//                             if let Some(entity) = unit_store.get_unit(position) {
//                                 println!("-- unit selected: {:?}", entity);
//
//                                 let team = action_query
//                                     .get_component::<Team>(*entity)
//                                     .unwrap();
//
//                                 if team.eq(&game_state.active_team) {
//                                     println!("Unit for active team selected");
//                                     store.state = UIState::UnitSelected(*entity);
//                                 } else {
//                                     println!("Inactive unit selected");
//                                 }
//                             }
//                         },
//                         UIState::UnitSelected(entity) => {
//                             let actions = action_query.get_component::<Actions>(entity.clone()).unwrap();
//                             let action = actions.get(0).unwrap();
//
//                             if is_action_valid(action, &entity, position, &unit_store, &action_query) {
//                                 println!("Execute action {:?}", entity);
//                                 cmds.send(UnitCmd::ExecuteAction(entity, 0, *position));
//                             }
//
//                             store.state = UIState::Default;
//                         }
//                     }
//                 }
//                 Interaction::Hovered => {
//                     println!("Hover {:?}", position);
//                 }
//                 Interaction::None => {
//                     // println!("None {:?}", position);
//                 }
//             }
//         }
//     }
// }

