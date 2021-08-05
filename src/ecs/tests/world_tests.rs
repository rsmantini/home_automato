use super::super::world;
use super::super::components::schedule::*;
use super::super::components::activation_time::*;

#[test]
fn new_entity() {
    let mut world =  world::World::new();
    let e0 = world.new_entity();
    let e1 = world.new_entity();
    assert_ne!(e0, e1);
}

#[test]
fn add_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    {
        let schedule = world.add_component::<Schedule>(e0);
        schedule.hour = 21;
    }
    let schedule = world.get_component::<Schedule>(e0);
    assert!(!schedule.is_none());
    assert_eq!(schedule.unwrap().hour, 21);
}

#[test]
fn add_existing_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    {
        let schedule = world.add_component::<Schedule>(e0);
        schedule.hour = 42;
    }
    let schedule = world.add_component::<Schedule>(e0);
    assert_eq!(schedule.hour, 42);
}

#[test]
fn remove_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    world.add_component::<Schedule>(e0);
    {
        let schedule = world.get_component::<Schedule>(e0);
        assert!(!schedule.is_none());
    }
    world.remove_component::<Schedule>(e0);
    let schedule = world.get_component::<Schedule>(e0);
    assert!(schedule.is_none());
}

#[test]
fn remove_entity() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    world.add_component::<Schedule>(e0);
    world.add_component::<ActivationTime>(e0);
    {
        let schedule = world.get_component::<Schedule>(e0);
        assert!(!schedule.is_none());
        let activation_time = world.get_component::<ActivationTime>(e0);
        assert!(!activation_time.is_none());
    }
    world.remove_entity(e0);
    let schedule = world.get_component::<Schedule>(e0);
    assert!(schedule.is_none());
    let activation_time = world.get_component::<ActivationTime>(e0);
    assert!(activation_time.is_none());
}

#[test]
fn multiple_entities() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    let e1 = world.new_entity();
    {
        let s0 = world.add_component::<Schedule>(e0);
        s0.min = 10;
        let s1 = world.add_component::<Schedule>(e1);
        s1.min = 30;
    }
    {
        let s0 = world.get_component::<Schedule>(e0);
        assert!(!s0.is_none());
        assert_eq!(s0.unwrap().min, 10);

        let s1 = world.get_component::<Schedule>(e1);
        assert!(!s1.is_none());
        assert_eq!(s1.unwrap().min, 30);
    }

    world.add_component::<ActivationTime>(e1);
    assert!(world.get_component::<ActivationTime>(e0).is_none());
    assert!(!world.get_component::<ActivationTime>(e1).is_none());

    world.remove_entity(e1);
    assert!(!world.get_component::<Schedule>(e0).is_none());
    assert!(world.get_component::<Schedule>(e1).is_none());
    assert!(world.get_component::<ActivationTime>(e1).is_none());
}