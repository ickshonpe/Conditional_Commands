use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait ConditionalEntityCommandsExt {
    fn insert_if<C: Component>(
        &mut self,
        cond: bool,
        component: C,
    ) -> &mut Self;

    fn insert_bundle_if<B: Bundle>(
        &mut self,
        cond: bool,
        component: B,
    ) -> &mut Self;

    fn with_children_if(
        &mut self,
        cond: bool,
        child_builder: impl FnOnce(&mut ChildBuilder)
    ) -> &mut Self;
}

impl ConditionalEntityCommandsExt for EntityCommands<'_, '_, '_> {
    fn insert_if<C: Component>(
        &mut self,
        cond: bool,
        component: C,
    ) -> &mut Self {
        if cond {
            self.insert(component);
        }
        self
    }

    fn insert_bundle_if<B: Bundle>(
        &mut self,
        cond: bool,
        bundle: B,
    ) -> &mut Self {
        if cond {
            self.insert_bundle(bundle);
        }
        self
    }

    fn with_children_if(
        &mut self,
        cond: bool,
        child_builder: impl FnOnce(&mut ChildBuilder)
    ) -> &mut Self {
        if cond {
            self.with_children(child_builder);
        }
        self
    }
}

