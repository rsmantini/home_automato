use super::components::*;
use std::any::Any;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Entity {
    id: i32,
}

pub trait Component: Sized {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)>;
}

pub trait System: Any {
    fn process(&mut self); 
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct World {
    entity_count: i32,
    schedules: Vec<(i32, schedule::Schedule)>,
    activation_times: Vec<(i32, activation_time::ActivationTime)>,
    systems: Vec<Box<dyn System>>
}

impl PartialEq for Box<dyn System> {
    fn eq(&self, other: &Box<dyn System>) -> bool {
        self.box_eq(other.as_any())
    }
}

impl Component for activation_time::ActivationTime {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)> {
        &mut world.activation_times
    }
}

impl Component for schedule::Schedule {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)> {
        &mut world.schedules
    }
}

impl World {
    pub fn new() -> World {
        World {
            entity_count: 0,
            schedules: Vec::new(),
            activation_times: Vec::new(),
            systems: Vec::new()
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let id = self.entity_count;
        self.entity_count += 1;
        if self.entity_count == core::i32::MAX {
            panic!("maximum number of entities reached");
        }
        Entity { id }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.schedules.retain(|x| x.0 != entity.id);
        self.activation_times.retain(|x| x.0 != entity.id);
    }

    pub fn add_component<V: Component + Default>(&mut self, entity: Entity) -> &mut V {
        let components = V::get(self);
        if let Some(i) = (0..components.len()).find(|&i| components[i].0 == entity.id) {
            &mut components[i].1
        } else {
            components.push((entity.id, V::default()));
            &mut components.last_mut().unwrap().1
        }
    }

    pub fn remove_component<C: Component>(&mut self, entity: Entity) {
        let components = C::get(self);
        components.retain(|x| x.0 != entity.id);
    }

    pub fn get_component<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let components = C::get(self);
        let tuple = components.iter_mut().find(|x| x.0 == entity.id);
        match tuple {
            Some(t) => Some(&mut t.1),
            None => None,
        }
    }

    pub fn add_system<S: System>(&mut self, system: S) {
        let s = Box::new(system);
        self.remove_system::<S>();
        self.systems.push(s)
    }

    pub fn remove_system<S: System>(&mut self) {
        self.systems.retain(|x| !(*x).as_any().is::<S>());
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.process();
        }
    }
}