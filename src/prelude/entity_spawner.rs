use bevy::prelude::*;

/// Structs should implement these traits to spawn all of their entities & components.
pub trait SpawnWithCommands {
    fn spawn_with_commands(self, commands: &mut Commands) -> &mut Commands;
}

pub trait SpawnWithChildBuilder {
    fn spawn_with_child_builder<'a, 'b>(self, commands: &'a mut ChildBuilder<'b>) -> &'a mut ChildBuilder<'b>;
}