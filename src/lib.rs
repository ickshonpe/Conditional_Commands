use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;

pub trait ConditionalInsertBundleExt {
    fn insert_if<B: Bundle>(&mut self, condition: bool, bundle: impl FnOnce() -> B) -> &mut Self;

    fn insert_if_else<A: Bundle, B: Bundle>(
        &mut self,
        condition: bool,
        bundle: impl FnOnce() -> A,
        else_bundle: impl FnOnce() -> B,
    ) -> &mut Self;

    fn insert_some<B: Bundle>(&mut self, optional_bundle: Option<B>) -> &mut Self;

    fn insert_some_or<A: Bundle, B: Bundle>(
        &mut self,
        optional_bundle: Option<A>,
        otherwise: impl FnOnce() -> B,
    ) -> &mut Self;
}

pub trait ConditionalChildBuilderExt {
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut ChildBuilder),
    ) -> &mut Self;
}

pub trait ConditionalWorldChildBuilderExt {
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut WorldChildBuilder),
    ) -> &mut Self;
}

macro_rules! ImplExt {
    ($t:ty) => {
        impl ConditionalInsertBundleExt for $t {
            /// if `condition`, add a [`Bundle`] to the entity
            fn insert_if<C: Bundle>(
                &mut self,
                condition: bool,
                bundle: impl FnOnce() -> C,
            ) -> &mut Self {
                if condition {
                    self.insert(bundle());
                }
                self
            }

            /// if `condition`, add a [`Bundle`] to the entity
            /// else add other [`Bundle`] to the entity
            fn insert_if_else<C: Bundle, D: Bundle>(
                &mut self,
                condition: bool,
                bundle: impl FnOnce() -> C,
                else_bundle: impl FnOnce() -> D,
            ) -> &mut Self {
                if condition {
                    self.insert(bundle())
                } else {
                    self.insert(else_bundle())
                }
            }

            /// If present, insert the inner value of `optional_Bundle`
            /// otherwise insert the Bundle returned by otherwise
            fn insert_some_or<C: Bundle, D: Bundle>(
                &mut self,
                optional_bundle: Option<C>,
                otherwise: impl FnOnce() -> D,
            ) -> &mut Self {
                if let Some(bundle) = optional_bundle {
                    self.insert(bundle);
                } else {
                    self.insert(otherwise());
                }
                self
            }

            /// If present, insert the inner value of `optional_bundle`
            fn insert_some<B: Bundle>(&mut self, optional_bundle: Option<B>) -> &mut Self {
                if let Some(bundle) = optional_bundle {
                    self.insert(bundle);
                }
                self
            }
        }
    };
}

ImplExt!(EntityCommands<'_, '_, '_>);
ImplExt!(EntityMut<'_>);

impl ConditionalChildBuilderExt for EntityCommands<'_, '_, '_> {
    /// if `condition`, create a [`ChildBuilder`] for the entity
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut ChildBuilder),
    ) -> &mut Self {
        if condition {
            self.with_children(child_builder);
        }
        self
    }
}

impl ConditionalWorldChildBuilderExt for EntityMut<'_> {
    /// if `condition`, create a [`WorldChildBuilder`] for the entity
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut WorldChildBuilder),
    ) -> &mut Self {
        if condition {
            self.with_children(child_builder);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn some() {
        #[derive(Component)]
        struct MyComponent;

        #[derive(Component, Default)]
        struct OtherComponent;

        #[derive(Component)]
        struct MyThing;

        #[derive(Component, Default)]
        struct AlternativeThing;

        fn spawn_entities(mut commands: Commands) {
            commands
                .spawn(MyComponent)
                .insert_some(Some(OtherComponent::default()))
                .insert_some_or(None::<MyThing>, AlternativeThing::default);
        }

        let mut app = App::new();
        app.add_startup_system(spawn_entities);
        app.update();
        let world = &mut app.world;
        let mut query = world.query_filtered::<Entity, (
            With<MyComponent>,
            With<OtherComponent>,
            Without<MyThing>,
            With<AlternativeThing>,
        )>();
        assert!(query.iter(&world).count() == 1);
    }
}
