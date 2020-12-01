use bevy::prelude::*;

pub trait EntitySpawner {
    fn spawn(self, commands: &mut Commands);
}

pub trait ChildEntitySpawner {
    fn spawn_as_child(self, commands: &mut ChildBuilder);
}