use bevy::prelude::*;

pub trait EntitySpawner {
    fn spawn(self, commands: &mut Commands) -> &mut Commands;
}

pub trait ChildEntitySpawner {
    fn spawn_as_child<'a, 'b>(self, commands: &'a mut ChildBuilder<'b>) -> &'a mut ChildBuilder<'b>;
}
