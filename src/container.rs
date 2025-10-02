use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use bevy_ecs::entity::{Entity, EntityHashSet};
use smallvec::{SmallVec, smallvec};

/// A container for the other entities that this entity is related to.
///
/// These containers store the relationship data, and is held inside the
/// [`Related`] component.
///
/// [`Related`]: crate::related::Related
pub trait EntityContainer: Clone + PartialEq + Eq + Debug + Send + Sync + 'static {
    /// Creates a new entity container with the initial given entity.
    fn new(entity: Entity) -> Self;

    /// Returns `true` if this entity is not related to any other entities.
    fn is_empty(&self) -> bool;

    /// Returns `true` if the given entity is related to this entity.
    fn contains(&self, entity: Entity) -> bool;

    /// Adds the given entity to the list of entities that this entity is related to.
    fn push(&mut self, entity: Entity);

    /// Removes the given entity from the list of entities that this entity is related to.
    fn remove(&mut self, entity: Entity);

    /// Consumes the entity container and returns an iterator over the entities
    /// that this entity is related to.
    fn into_iter(self) -> impl Iterator<Item = Entity>;

    /// Returns an iterator over the entities that this entity is related to.
    fn iter(&self) -> impl Iterator<Item = Entity>;
}

impl EntityContainer for Entity {
    fn new(entity: Entity) -> Self {
        entity
    }

    fn is_empty(&self) -> bool {
        *self == Entity::PLACEHOLDER
    }

    fn contains(&self, entity: Entity) -> bool {
        *self == entity
    }

    fn push(&mut self, entity: Entity) {
        *self = entity;
    }

    fn remove(&mut self, entity: Entity) {
        if *self == entity {
            *self = Entity::PLACEHOLDER;
        }
    }

    fn into_iter(self) -> impl Iterator<Item = Entity> {
        std::iter::once(self)
    }

    fn iter(&self) -> impl Iterator<Item = Entity> {
        std::iter::once(*self)
    }
}

impl<const N: usize> EntityContainer for SmallVec<[Entity; N]> {
    fn new(entity: Entity) -> Self {
        smallvec![entity]
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.as_slice().contains(&entity)
    }

    fn push(&mut self, entity: Entity) {
        self.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        self.retain(|&mut id| id != entity);
    }

    fn into_iter(self) -> impl Iterator<Item = Entity> {
        IntoIterator::into_iter(self)
    }

    fn iter(&self) -> impl Iterator<Item = Entity> {
        self.as_slice().iter().copied()
    }
}

impl EntityContainer for Vec<Entity> {
    fn new(entity: Entity) -> Self {
        vec![entity]
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.as_slice().contains(&entity)
    }

    fn push(&mut self, entity: Entity) {
        self.push(entity);
    }

    fn remove(&mut self, entity: Entity) {
        self.retain(|&id| id != entity);
    }

    fn into_iter(self) -> impl Iterator<Item = Entity> {
        IntoIterator::into_iter(self)
    }

    fn iter(&self) -> impl Iterator<Item = Entity> {
        self.as_slice().iter().copied()
    }
}

impl EntityContainer for EntityHashSet {
    fn new(entity: Entity) -> Self {
        let mut set = EntityHashSet::default();
        set.insert(entity);
        set
    }

    fn is_empty(&self) -> bool {
        self.deref().is_empty()
    }

    fn contains(&self, entity: Entity) -> bool {
        self.deref().contains(&entity)
    }

    fn push(&mut self, entity: Entity) {
        self.insert(entity);
    }

    fn remove(&mut self, entity: Entity) {
        self.deref_mut().remove(&entity);
    }

    fn into_iter(self) -> impl Iterator<Item = Entity> {
        IntoIterator::into_iter(self)
    }

    fn iter(&self) -> impl Iterator<Item = Entity> {
        self.iter().copied()
    }
}
