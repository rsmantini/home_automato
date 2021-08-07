use super::super::components::*;
use super::super::world;

#[test]
fn new_entity() {
    let mut world = world::World::new();
    let e0 = world.new_entity();
    let e1 = world.new_entity();
    assert_ne!(e0, e1);
}

#[test]
fn add_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();

    let mut schedule = Schedule::default();
    schedule.hour = 21;
    world.add_component::<Schedule>(e0, schedule);

    let schedule = world.get_component::<Schedule>(e0);
    assert!(!schedule.is_none());
    assert_eq!(schedule.unwrap().hour, 21);
}

#[test]
fn add_existing_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();

    let mut schedule = Schedule::default();
    schedule.min = 42;
    world.add_component::<Schedule>(e0, schedule);

    let mut schedule = Schedule::default();
    schedule.min = 17;
    world.add_component::<Schedule>(e0, schedule);

    let schedule = world.get_component::<Schedule>(e0);
    assert!(!schedule.is_none());
    assert_eq!(schedule.unwrap().min, 17);
}

#[test]
fn remove_component() {
    let mut world = world::World::new();
    let e0 = world.new_entity();

    let schedule = Schedule::default();
    world.add_component::<Schedule>(e0, schedule);

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

    let schedule = Schedule::default();
    world.add_component::<Schedule>(e0, schedule);
    let activation_time = ActivationTime::default();
    world.add_component::<ActivationTime>(e0, activation_time);

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

    let mut s0 = Schedule::default();
    s0.min = 10;
    world.add_component::<Schedule>(e0, s0);
    let mut s1 = Schedule::default();
    s1.min = 30;
    world.add_component::<Schedule>(e1, s1);

    {
        let s0 = world.get_component::<Schedule>(e0);
        assert!(!s0.is_none());
        assert_eq!(s0.unwrap().min, 10);

        let s1 = world.get_component::<Schedule>(e1);
        assert!(!s1.is_none());
        assert_eq!(s1.unwrap().min, 30);
    }

    let activation_time = ActivationTime::default();
    world.add_component::<ActivationTime>(e1, activation_time);
    assert!(world.get_component::<ActivationTime>(e0).is_none());
    assert!(!world.get_component::<ActivationTime>(e1).is_none());

    world.remove_entity(e1);
    assert!(!world.get_component::<Schedule>(e0).is_none());
    assert!(world.get_component::<Schedule>(e1).is_none());
    assert!(world.get_component::<ActivationTime>(e1).is_none());
}
