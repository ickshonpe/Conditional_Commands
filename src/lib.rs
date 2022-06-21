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

    fn insert_some<C: Component>(
        &mut self,
        optional_component: Option<C>
    ) -> &mut Self;

    fn insert_some_or_else<C: Component, D: Component>(
        &mut self,
        optional_component: Option<C>,
        otherwise: impl FnOnce() -> D
    ) -> &mut Self;

    fn insert_bundle_some<B: Bundle>(
        &mut self,
        optional_bundle: Option<B>
    ) -> &mut Self;

    fn insert_bundle_some_or_else<B: Bundle, A: Bundle>(
        &mut self,
        optional_bundle: Option<B>,
        otherwise: impl FnOnce() -> A
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

    /// If present, insert the inner value of `optional_component`
    fn insert_some<C: Component>(
        &mut self,
        optional_component: Option<C>
    ) -> &mut Self {        
        if let Some(component) = optional_component {
            self.insert(component);
        }
        self
    }

     /// If present, insert the inner value of `optional_component`
     /// otherwise insert the component returned by otherwise
     fn insert_some_or_else<C: Component, D: Component>(
        &mut self,
        optional_component: Option<C>,
        otherwise: impl FnOnce() -> D,
    ) -> &mut Self {        
        if let Some(component) = optional_component {
            self.insert(component);
        } else {
            self.insert((otherwise)());
        }
        self
    }

    /// If present, insert the inner value of `optional_bundle`
    fn insert_bundle_some<B: Bundle>(
        &mut self,
        optional_bundle: Option<B>
    ) -> &mut Self {
        if let Some(bundle) = optional_bundle {
            self.insert_bundle(bundle);
        }
        self
    }

    /// If present, insert the inner value of `optional_bundle`
    /// otherwise insert the component returned by otherwise
    fn insert_bundle_some_or_else<B: Bundle, A: Bundle>(
        &mut self,
        optional_bundle: Option<B>,
        otherwise: impl FnOnce() -> A
    ) -> &mut Self {
        if let Some(bundle) = optional_bundle {
            self.insert_bundle(bundle);
        } else {
            self.insert_bundle((otherwise)());
        }
        self
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

    /// If present, insert the inner value of `optional_component`
    fn insert_some<C: Component>(
        &mut self,
        optional_component: Option<C>
    ) -> &mut Self {
        if let Some(component) = optional_component {
            self.insert(component);
        }
        self
    }

    /// If present, insert the inner value of `optional_component`
    /// otherwise insert the component returned by otherwise
    fn insert_some_or_else<C: Component, D: Component>(
        &mut self,
        optional_component: Option<C>,
        otherwise: impl FnOnce() -> D,
    ) -> &mut Self {        
        if let Some(component) = optional_component {
            self.insert(component);
        } else {
            self.insert((otherwise)());
        }
        self
    }

    /// If present, insert the inner value of `optional_bundle`
    fn insert_bundle_some<B: Bundle>(
        &mut self,
        optional_bundle: Option<B>
    ) -> &mut Self {
        if let Some(bundle) = optional_bundle {
            self.insert_bundle(bundle);
        }
        self
    }

    /// If present, insert the inner value of `optional_bundle`
    /// otherwise insert the component returned by otherwise
    fn insert_bundle_some_or_else<B: Bundle, A: Bundle>(
        &mut self,
        optional_bundle: Option<B>,
        otherwise: impl FnOnce() -> A
    ) -> &mut Self {
        if let Some(bundle) = optional_bundle {
            self.insert_bundle(bundle);
        } else {
            self.insert_bundle((otherwise)());
        }
        self
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
