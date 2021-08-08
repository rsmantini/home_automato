use super::components::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Entity {
    id: i32,
}

pub struct World {
    entity_count: i32,
    entities: Vec<Entity>,
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
