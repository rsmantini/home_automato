use super::*;

#[test]
fn new_entity() {
    let mut world = home_automation::world::World::new();
    let e0 = world.new_entity();
    let e1 = world.new_entity();
    assert_ne!(e0, e1);
}