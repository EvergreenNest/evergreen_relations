use std::{any::type_name, marker::PhantomData};

use bevy_ecs::{
    component::{Component, ComponentHooks, ComponentId, Immutable, StorageType},
    entity::Entity,
    event::Events,
    world::{DeferredWorld, World},
};
use bevy_ecs::component::{ComponentHook, HookContext};
use crate::{container::EntityContainer, event::RelationEvent, relation::Relatable};

/// [`Component`] used to store [`Relation`] data for a given side of a relationship,
/// i.e. the [`Relatable`].
///
/// [`Relation`]: crate::relation::Relation
pub struct Related<N: Relatable> {
    pub(crate) container: N::Container,
}

impl<N: Relatable> Component for Related<N> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Immutable;

    /*fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(associate::<N>);
        hooks.on_replace(disassociate::<N>);
        hooks.on_remove(disassociate::<N>);
    }*/

    fn on_insert() -> Option<ComponentHook> {
        Some(associate::<N>)
    }

    fn on_replace() -> Option<ComponentHook> {
        Some(disassociate::<N>)
    }

    fn on_remove() -> Option<ComponentHook> {
        Some(disassociate::<N>)
    }
}

impl<N: Relatable> Related<N> {
    pub fn new(node: impl Into<N::Container>) -> Self {
        Self {
            container: node.into(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.container.iter()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.container.contains(entity)
    }
}

impl<N: Relatable> Clone for Related<N> {
    fn clone(&self) -> Self {
        Self {
            container: self.container.clone(),
        }
    }
}

impl<N: Relatable> PartialEq for Related<N> {
    fn eq(&self, other: &Self) -> bool {
        self.container == other.container
    }
}

impl<N: Relatable> Eq for Related<N> {}

impl<N: Relatable> std::fmt::Debug for Related<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Related")
            .field(&type_name::<N>())
            .field(&self.container)
            .finish()
    }
}

impl<N: Relatable> From<Entity> for Related<N> {
    fn from(entity: Entity) -> Self {
        Self {
            container: N::Container::new(entity),
        }
    }
}

impl<N: Relatable> FromIterator<Entity> for Related<N>
where
    N::Container: FromIterator<Entity>,
{
    fn from_iter<T: IntoIterator<Item = Entity>>(iter: T) -> Self {
        Self {
            container: N::Container::from_iter(iter),
        }
    }
}

fn associate<N: Relatable>(mut world: DeferredWorld, HookContext { entity: a_id, .. }: HookContext ) {
    world.commands().queue(move |world: &mut World| {
        // Get the IDs of the other entities that this entity is related to.
        let Some(a_related) = world.get::<Related<N>>(a_id).cloned() else {
            return;
        };

        // For each other related entity, associate them with this entity.
        for b_id in a_related.iter() {
            let Ok(mut b) = world.get_entity_mut(b_id) else {
                return;
            };

            let b_related = b.get::<Related<N::Opposite>>();

            let b_points_to_a = b_related.is_some_and(|b| b.contains(a_id));
            if !b_points_to_a {
                if let Some(b_related) = b_related {
                    // The other entity is already related to some entities, so add this entity to the list.
                    let mut b_related = b_related.clone();
                    b_related.container.push(a_id);
                    b.insert(b_related);
                } else {
                    // The other entity is not yet related to any entities, so relate it to this entity.
                    let b_related = Related::<N::Opposite>::from(a_id);
                    b.insert(b_related);
                }

                if let Some(mut events) =
                    world.get_resource_mut::<Events<RelationEvent<N::Relation>>>()
                {
                    events.send(RelationEvent::Added(a_id, b_id, PhantomData));
                }
            }
        }
    });
}

fn disassociate<N: Relatable>(mut world: DeferredWorld, HookContext { entity: a_id, .. }: HookContext) {
    // Gets the IDs of the entities that this entity is no longer related to.
    let Some(b_ids) = world.get::<Related<N>>(a_id).cloned() else {
        return;
    };

    world.commands().queue(move |world: &mut World| {
        // For each related entity, disassociate it from this entity.
        for b_id in b_ids.iter() {
            let a_points_to_b = world
                .get::<Related<N>>(a_id)
                .is_some_and(|a_related| a_related.contains(b_id));

            let Ok(mut b) = world.get_entity_mut(b_id) else {
                return;
            };

            let b_related = b.get::<Related<N::Opposite>>();

            let b_points_to_a = b_related.is_some_and(|b| b.contains(a_id));
            if b_points_to_a && !a_points_to_b {
                if let Some(b_related) = b_related {
                    // The other entity is related to some entities, so make sure this entity is removed from the list.
                    let mut b_related = b_related.clone();
                    b_related.container.remove(a_id);

                    // If the other entity is no longer related to any entities, remove the component.
                    if b_related.container.is_empty() {
                        b.remove::<Related<N::Opposite>>();
                    } else {
                        b.insert(b_related);
                    }
                }

                if let Some(mut events) =
                    world.get_resource_mut::<Events<RelationEvent<N::Relation>>>()
                {
                    events.send(RelationEvent::Removed(a_id, b_id, PhantomData));
                }
            }
        }
    });
}
