use super::components::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Entity {
    id: i32,
}

impl Entity {
    pub fn new(id: i32) -> Entity {
        Entity { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Default)]
pub struct World {
    entity_count: i32,
    pub entities: Vec<Entity>,
    pub components: Components,
}

impl World {
    pub fn new() -> World {
        World {
            entity_count: 0,
            entities: Vec::new(),
            components: Components::default(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        if self.entity_count == core::i32::MAX {
            panic!("maximum number of entities reached");
        }
        let id = self.entity_count;
        self.entity_count += 1;
        self.components.push();
        self.entities.push(Entity { id });
        Entity { id }
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        if let Some(index) = self.get_index(entity) {
            self.entities.remove(index);
            self.components.remove(index);
        }
    }

    pub fn get_component<C: ComponentVec>(&mut self, entity: Entity) -> Option<&mut C> {
        match self.get_component_option::<C>(entity) {
            Ok(opt) => opt.as_mut(),
            Err(_) => None,
        }
    }

    pub fn add_component<C: ComponentVec>(&mut self, entity: Entity, component: C) {
        let opt = self.get_component_option::<C>(entity);
        *opt.expect("adding component to invalid entity") = Some(component);
    }

    pub fn remove_component<C: ComponentVec>(&mut self, entity: Entity) {
        if let Ok(opt) = self.get_component_option::<C>(entity) {
            *opt = None;
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.get_index(entity).is_some()
    }

    fn get_index(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|x| x.id == entity.id)
    }

    fn get_component_option<C: ComponentVec>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut Option<C>, ()> {
        let index = self.get_index(entity);
        match index {
            Some(i) => Ok(&mut C::get_vec(&mut self.components)[i]),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_entity() {
        let mut world = World::new();
        let e0 = world.new_entity();
        let e1 = world.new_entity();
        assert_ne!(e0, e1);
    }

    #[test]
    fn add_component() {
        let mut world = World::new();
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
        let mut world = World::new();
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
        let mut world = World::new();
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
        let mut world = World::new();
        let e0 = world.new_entity();

        let schedule = Schedule::default();
        world.add_component::<Schedule>(e0, schedule);
        let activation_time = ActivationState::ToBeScheduled;
        world.add_component::<ActivationState>(e0, activation_time);

        {
            let schedule = world.get_component::<Schedule>(e0);
            assert!(!schedule.is_none());
            let activation_time = world.get_component::<ActivationState>(e0);
            assert!(!activation_time.is_none());
        }

        world.remove_entity(e0);
        let schedule = world.get_component::<Schedule>(e0);
        assert!(schedule.is_none());
        let activation_time = world.get_component::<ActivationState>(e0);
        assert!(activation_time.is_none());
    }

    #[test]
    fn multiple_entities() {
        let mut world = World::new();
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

        let activation_time = ActivationState::ToBeScheduled;
        world.add_component::<ActivationState>(e1, activation_time);
        assert!(world.get_component::<ActivationState>(e0).is_none());
        assert!(!world.get_component::<ActivationState>(e1).is_none());

        world.remove_entity(e1);
        assert!(!world.get_component::<Schedule>(e0).is_none());
        assert!(world.get_component::<Schedule>(e1).is_none());
        assert!(world.get_component::<ActivationState>(e1).is_none());
    }
}
