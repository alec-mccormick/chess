## Chess

To run in debug mode:
```
# RUST_LOG=chess=debug cargo run
```

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
- [ ] Add Networking
    - [ ] Serialize/deserialize board
    - [ ] Serialize/deserialize messages 
- [ ] Migrate to using SpriteConfig
- [ ] Add Bot AI
- [ ] Show movable squares on hover
- [x] Add startup screen
    - [ ] Allow user to select team
- [ ] Build Action UI
    - [ ] Add Icon
    - [ ] Add Description
- [ ] Add State Machine
- [ ] Add events for action executed, etc.
- [ ] Update Action to no longer rely on Query
    - [ ] Maintain internal list of all units in UnitStore?
    - [ ] Migrate to ObjectId


### Gameplay Thoughts

- Explore mtg like synergies in tactical gameplay
- Consider what makes synergy preferable to "just good shit" squads.
    - large bloated reward systems tend to be less synergistic. Turning down rewards in STS
      is often better than taking them.

    
## Bevy Stages

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