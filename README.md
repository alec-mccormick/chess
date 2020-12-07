## Chess

This project's primary role is to serve as a framework for an isometric 
turn-based game written in Rust with Bevy engine.

Run the following commands to launch:
```
# Launch application with the title "Chess A" to act as the server.
$ cargo run -- --port 12351 --title "Chess A"

# Launch application with the title "Chess B" to act as the client & configure it to connect.
$ cargo run -- --port 12350 --title "Chess B" --remote "127.0.0.1:12351"
```


Set the environment variable RUST_LOG="chess=debug" for debug logs.


## Roadmap

- [x] GameState, swap turns with each move (needs to be fixed to not work on invalid moves)
- [x] Validate move before executing
- [x] Highlight moves
- [x] Show current player with text
- [x] Implement attacking
    - [x] Handle health changes in various modules (despawn + remove sprite)
- [x] Implement Knight
- [x] Implement King
- [x] Add robust logging
- [x] Update rendering to isometric
- [x] Add Networking
    - [x] Serialize/deserialize board
    - [x] Serialize/deserialize messages 
    - [ ] Add reconnection functionality
- [ ] Migrate to using SpriteConfig
- [ ] Migrate to animated sprites
- [ ] Show movable squares on hover
- [ ] Add Bot AI
- [ ] 
- [x] Add startup screen
    - [ ] Allow user to select team
- [ ] Build Action UI
    - [ ] Add Icon
    - [ ] Add Description
- [x] Add events for action executed, etc.
- [ ] Update Action to no longer rely on Query
    - [x] Maintain internal list of all units in UnitStore?
    - [x] Migrate to ObjectId

    
## Bevy Stages

This is just here for reference

```
app::FIRST
    + core::time_system
    + core::timer_system

app::PRE_EVENT

app::EVENT
    + input::keyboard_input_system
    + input::mouse_button_input_system
    + input::gamepad_event_system
    + input::touch_screen_input_system

scene::SCENE_STAGE
    + scene::scene_spawned_system

asset::LOAD_ASSETS
    + asset:filesystem_watcher_system

app::PRE_UPDATE
    + core::entity_labels_system    // Updates entity labels, used to id -> entity mapping
    + asset::free_unused_asets_system
    + render::clear_draw_system
    + ui::ui_focus_system

app::UPDATE

ui::UI
    ui::widget::text_system
    ui::widget::image_node_system
    ui::z_system
    ui::flex_node_system

app::POST_UPDATE
    + transform::transform_systems
    + render::active_cameras_system
    + render::camera_system
    + render::camera_visible_entities
    + sprite::sprite_system
    + sprite::asset_shader_defs_system
    + audio::play_queued_audio

asset::ASSET_EVENTS

render::RENDER_RESOURCE
    + render::mesh_resource_profider_system
    + render::texture_resource_system

render::RENDER_GRAPH_SYSTEMS
    + render::render_graph_systems

render::DRAW
    + render::draw_render_pipelines_system
    + ui::widget::draw_text_system

render::RENDER

render::POST_RENDER
    + render::shader::clear_shader_defs_system


app::LAST
```