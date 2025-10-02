use core::fmt;
use std::marker::PhantomData;

use bevy_ecs::prelude::*;

use crate::relation::Relation;

/// A [`Message`] that is emitted when a [`Relation`] is added or removed between
/// two entities.
#[derive(Message)]
pub enum RelationEvent<R: Relation> {
    Added(Entity, Entity, PhantomData<fn(R)>),
    Removed(Entity, Entity, PhantomData<fn(R)>),
}

impl<R: Relation> fmt::Debug for RelationEvent<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Added(arg0, arg1, _) => f.debug_tuple("Added").field(arg0).field(arg1).finish(),
            Self::Removed(arg0, arg1, _) => {
                f.debug_tuple("Removed").field(arg0).field(arg1).finish()
            }
        }
    }
}

impl<R: Relation> PartialEq for RelationEvent<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Added(l0, l1, _), Self::Added(r0, r1, _))
            | (Self::Removed(l0, l1, _), Self::Removed(r0, r1, _)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

impl<R: Relation> Eq for RelationEvent<R> {}

impl<R: Relation> Clone for RelationEvent<R> {
    fn clone(&self) -> Self {
        match self {
            Self::Added(arg0, arg1, _) => Self::Added(*arg0, *arg1, PhantomData),
            Self::Removed(arg0, arg1, _) => Self::Removed(*arg0, *arg1, PhantomData),
        }
    }
}
