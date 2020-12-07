use bevy::prelude::*;

/// Structs should implement these traits to spawn all of their entities & components.
pub trait SpawnWithCommands {
    fn spawn_with_commands(self, commands: &mut Commands) -> &mut Commands;
}

pub trait SpawnWithChildBuilder {
    fn spawn_with_child_builder<'a, 'b>(self, commands: &'a mut ChildBuilder<'b>) -> &'a mut ChildBuilder<'b>;
}


/// Extension of bevy::ecs::Commands & similar structs to allow them to spawn
/// objects which implement the SpawnWithCommands trait.
pub trait SpawnWithEntitySpawner {
    fn spawn_with_entity_spawner<T>(&mut self, spawner: T) -> &mut Self;
}

impl SpawnWithEntitySpawner for Commands {
    fn spawn_with_entity_spawner<T>(&mut self, spawner: T) -> &mut Self
        where T: SpawnWithCommands
    {
        spawner.spawn_with_commands(self);
        self
    }
}


impl SpawnWithEntitySpawner for ChildBuilder {
    fn spawn_with_entity_spawner<T>(&mut self, spawner: T) -> &mut Self
        where T: SpawnWithChildBuilder
    {
        spawner.spawn_with_child_builder(self);
        self
    }
}