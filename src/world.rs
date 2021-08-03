#[derive(Copy, Clone, PartialEq, Default)]
struct Schedule {
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub repeat: bool,
}

#[derive(Copy, Clone, PartialEq, Default)]
struct ActivationTime {
    pub seconds_to_acivate: u32,
}

#[derive(Copy, Clone, PartialEq, Default)]
struct Entity {
    id: i32,
}

struct World {
    entity_count: i32,
    schedules: Vec<(i32, Schedule)>,
    activation_times: Vec<(i32, ActivationTime)>,
}

trait Component: Sized {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)>;
}

impl Component for Schedule {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)> {
        &mut world.schedules
    }
}

impl Component for ActivationTime {
    fn get<'a>(world: &'a mut World) -> &'a mut Vec<(i32, Self)> {
        &mut world.activation_times
    }
}

impl World {
    pub fn new() -> World {
        World {
            entity_count: 0,
            schedules: Vec::new(),
            activation_times: Vec::new(),
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

    pub fn remove_component<V: Component>(&mut self, entity: Entity) {
        let components = V::get(self);
        components.retain(|x| x.0 != entity.id);
    }

    pub fn get_component<V: Component>(&mut self, entity: Entity) -> Option<&mut V> {
        let components = V::get(self);
        let tuple = components.iter_mut().find(|x| x.0 == entity.id);
        match tuple {
            Some(t) => Some(&mut t.1),
            None => None,
        }
    }
}
