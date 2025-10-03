use std::marker::PhantomData;

use bevy_ecs::prelude::*;

use evergreen_relations::{
    message::RelationMessage,
    related::Related,
    relation::{Relatable, Relation},
};

/// An undirected 1:1 relationship between entities.
#[derive(Relation)]
#[relation(source = SignificantOtherOf, target = SignificantOtherOf)]
pub struct Marriage;

pub type SignificantOther = Related<SignificantOtherOf>;

#[derive(Relatable)]
#[relatable(Entity in Marriage, opposite = SignificantOtherOf)]
pub struct SignificantOtherOf;

#[test]
fn add_remove() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn(SignificantOther::new(a)).id();

    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(b))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(a))
    );

    world.entity_mut(b).remove::<SignificantOther>();
    world.flush();

    assert_eq!(world.get::<SignificantOther>(a), None);
    assert_eq!(world.get::<SignificantOther>(b), None);
}

#[test]
fn triangle_break_up() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn(SignificantOther::new(a)).id();
    let c = world.spawn_empty().id();

    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(b))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(a))
    );
    assert_eq!(world.get::<SignificantOther>(c), None);

    world.entity_mut(a).insert(SignificantOther::new(c));
    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(c))
    );
    assert_eq!(world.get::<SignificantOther>(b), None);
    assert_eq!(
        world.get::<SignificantOther>(c),
        Some(&SignificantOther::new(a))
    );
}

#[test]
fn add_repeated() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn_empty().id();

    world.entity_mut(a).insert(SignificantOther::new(b));
    world.entity_mut(a).insert(SignificantOther::new(b));
    world.entity_mut(a).insert(SignificantOther::new(b));
    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(b))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(a))
    );
}

#[test]
fn swap() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn_empty().id();
    let c = world.spawn_empty().id();
    let d = world.spawn_empty().id();

    world.entity_mut(a).insert(SignificantOther::new(b));
    world.entity_mut(c).insert(SignificantOther::new(d));
    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(b))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(a))
    );
    assert_eq!(
        world.get::<SignificantOther>(c),
        Some(&SignificantOther::new(d))
    );
    assert_eq!(
        world.get::<SignificantOther>(d),
        Some(&SignificantOther::new(c))
    );

    world.entity_mut(a).insert(SignificantOther::new(c));
    world.entity_mut(b).insert(SignificantOther::new(d));
    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(c))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(d))
    );
    assert_eq!(
        world.get::<SignificantOther>(c),
        Some(&SignificantOther::new(a))
    );
    assert_eq!(
        world.get::<SignificantOther>(d),
        Some(&SignificantOther::new(b))
    );
}

#[test]
fn looped_add_remove() {
    let mut world = World::new();

    let a = world.spawn_empty().id();
    let b = world.spawn_empty().id();

    for _ in 0..10_000 {
        world.entity_mut(a).insert(SignificantOther::new(b));
        world.entity_mut(a).remove::<SignificantOther>();
    }

    world.flush();

    assert_eq!(world.get::<SignificantOther>(a), None);
    assert_eq!(world.get::<SignificantOther>(b), None);
}

#[test]
fn events() {
    let mut world = World::new();

    world.init_resource::<Messages<RelationMessage<Marriage>>>();

    let a = world.spawn_empty().id();
    let b = world.spawn(SignificantOther::new(a)).id();
    world.flush();

    assert_eq!(
        world.get::<SignificantOther>(a),
        Some(&SignificantOther::new(b))
    );
    assert_eq!(
        world.get::<SignificantOther>(b),
        Some(&SignificantOther::new(a))
    );

    assert_eq!(
        world
            .resource_mut::<Messages<RelationMessage<Marriage>>>()
            .drain()
            .collect::<Vec<_>>(),
        vec![RelationMessage::<Marriage>::Added(b, a, PhantomData)]
    );

    world.entity_mut(a).remove::<SignificantOther>();
    world.flush();

    assert_eq!(world.get::<SignificantOther>(a), None);
    assert_eq!(world.get::<SignificantOther>(b), None);

    assert_eq!(
        world
            .resource_mut::<Messages<RelationMessage<Marriage>>>()
            .drain()
            .collect::<Vec<_>>(),
        vec![RelationMessage::<Marriage>::Removed(a, b, PhantomData)]
    );
}
