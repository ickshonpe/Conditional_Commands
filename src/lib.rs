use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;

pub trait ConditionalInsertComponentsExt {
    fn insert_if<C: Component>(
        &mut self,
        condition: bool,
        component: C,
    ) -> &mut Self;

    fn insert_bundle_if<B: Bundle>(
        &mut self,
        condition: bool,
        bundle: B,
    ) -> &mut Self;

    fn insert_if_else<C: Component, D: Component>(
        &mut self,
        condition: bool,
        component: C,
        else_component: D,
    ) -> &mut Self;

    fn insert_bundle_if_else<A: Bundle, B: Bundle>(
        &mut self,
        condition: bool,
        bundle: A,
        else_bundle: B,
    ) -> &mut Self;
}

// Couldn't quite finesse the lifetimes etc to get a single generic child builder trait
// for both ['EntityCommands'] and ['EntityMut'].
// Love to hear from anyone who can figure it out.

pub trait ConditionalChildBuilderExt {
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut ChildBuilder)
    ) -> &mut Self;
}

pub trait ConditionalWorldChildBuilderExt {
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut WorldChildBuilder)
    ) -> &mut Self;
}

impl ConditionalInsertComponentsExt for EntityCommands<'_, '_, '_> {
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

    /// if `condition`, add a [`Component`] to the entity
    /// else add other [`Component`] to the entity
    fn insert_if_else<C: Component, D: Component>(
        &mut self,
        condition: bool,
        component: C,
        else_component: D,
    ) -> &mut Self {
        if condition {
            self.insert(component)
        } else {
            self.insert(else_component)
        } 
    }

    /// if `condition`, add a [`Bundle`] to the entity
    /// else add other [`Bundle`] to the entity
    fn insert_bundle_if_else<A: Bundle, B: Bundle>(
        &mut self,
        condition: bool,
        bundle: A,
        else_bundle: B,
    ) -> &mut Self {
        if condition {
            self.insert_bundle(bundle)
        } else {
            self.insert_bundle(else_bundle)
        } 
    }

    
}

impl ConditionalChildBuilderExt for EntityCommands<'_, '_, '_> {
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

impl ConditionalInsertComponentsExt for EntityMut<'_> {
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

    /// if `condition`, add a [`Component`] to the entity
    /// else add other [`Component`] to the entity
    fn insert_if_else<C: Component, D: Component>(
        &mut self,
        condition: bool,
        component: C,
        else_component: D,
    ) -> &mut Self {
        if condition {
            self.insert(component)
        } else {
            self.insert(else_component)
        } 
    }

    /// if `condition`, add a [`Bundle`] to the entity
    /// else add other [`Bundle`] to the entity
    fn insert_bundle_if_else<A: Bundle, B: Bundle>(
        &mut self,
        condition: bool,
        bundle: A,
        else_bundle: B,
    ) -> &mut Self {
        if condition {
            self.insert_bundle(bundle)
        } else {
            self.insert_bundle(else_bundle)
        } 
    }

    
}

impl ConditionalWorldChildBuilderExt for EntityMut<'_> {
    /// if `condition`, create a [`WorldChildBuilder`] for the entity
    fn with_children_if(
        &mut self,
        condition: bool,
        child_builder: impl FnOnce(&mut WorldChildBuilder)
    ) -> &mut Self {
        if condition {
            self.with_children(child_builder);
        }
        self
    }
}
