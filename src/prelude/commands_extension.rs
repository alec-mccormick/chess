use bevy::prelude::*;

pub trait CommandsExtension {

    fn spawn_multi_bundle<I>(&mut self, components_iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Bundle + Send + Sync + 'static;
}


impl CommandsExtension for Commands {
    fn spawn_multi_bundle<I>(&mut self, components_iter: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Bundle + Send + Sync + 'static
    {
        let mut iter = components_iter.into_iter();

        if let Some(components) = iter.next() {
            let entity = self.spawn(components)
                .current_entity()
                .expect("Entity should have been spawned successfully but current entity is None");

            for components in iter {
                self.insert(entity, components);
            }
        }

        self
    }
}