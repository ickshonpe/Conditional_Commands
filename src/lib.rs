use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait ConditionalEntityCommandsExt {
    fn insert_if<C: Component>(
        &mut self,
        condition: bool,
        component: C,
    ) -> &mut Self;

    fn insert_bundle_if<B: Bundle>(
        &mut self,
        condition: bool,
        component: B,
    ) -> &mut Self;

    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut ChildBuilder)
    ) -> &mut Self;
}

impl ConditionalEntityCommandsExt for EntityCommands<'_, '_, '_> {
    /// if `condition`, add a [`Component`] to the entity
    fn insert_if<C: Component>(
        &mut self,
        condition: bool,
        component: C,
    ) -> &mut Self {
        if condition {
            self.insert(component);
        }
        self
    }

    /// if `condition`, add a [`Bundle`] to the entity
    fn insert_bundle_if<B: Bundle>(
        &mut self,
        condition: bool,
        bundle: B,
    ) -> &mut Self {
        if condition {
            self.insert_bundle(bundle);
        }
        self
    }

    /// if `condition`, create a [`ChildBuilder`] for the entity
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut ChildBuilder)
    ) -> &mut Self {
        if condition {
            self.with_children(child_builder);
        }
        self
    }
}

