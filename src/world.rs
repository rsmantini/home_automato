use slotmap::{new_key_type, SlotMap};
 
new_key_type! {
    struct ScheduleKey;
}

struct Schedule {
    pub hour: i8,
    pub min: i8,
    pub sec: i8,
    pub repeat: bool,
}

new_key_type! {
    struct ActivationTimeKey;
}
struct ActivationTime {
    pub seconds_to_acivate: u32
}

struct Entity {
    id: i32,
    schedule: Option<ScheduleKey>,
    activation_time: Option<ActivationTimeKey>
}

struct World {
    entity_count: i32,
    schedules: SlotMap<ScheduleKey, Schedule>,
    activation_times: SlotMap<ActivationTimeKey, ActivationTime>,
    entities: Vec<Entity>
}

impl World {
    pub fn new() -> World {
        World{entity_count: 0, schedules: SlotMap::with_key(), activation_times: SlotMap::with_key(), entities: Vec::new()}
    }
    pub fn new_entity(&mut self) -> i32 {
        let id = self.entity_count;
        self.entities.push(Entity{id: id, schedule: None, activation_time: None});
        self.entity_count += 1;
        id
    }
    pub fn add_schedule(&mut self, entity_id: i32) -> Option<&Schedule> {
        let opt_entity = self.entities.iter_mut().find(|x| x.id == entity_id);
        match opt_entity {
            None => None,
            Some(entity) => {
                let key = match entity.schedule {
                    Some(k) => k,
                    None => {
                        let key = self.schedules.insert(Schedule{hour: 0, min: 0, sec: 0, repeat: false});
                        entity.schedule = Some(key);
                        key
                    } 
                };
                self.schedules.get(key)
            }
        }
    }
    pub fn add_activation_time(&mut self, entity_id: i32) -> Option<&ActivationTime> {
        let opt_entity = self.entities.iter_mut().find(|x| x.id == entity_id);
        match opt_entity {
            None => None,
            Some(entity) => {
                let key = match entity.activation_time{
                    Some(k) => k,
                    None => {
                        let key = self.activation_times.insert(ActivationTime{seconds_to_acivate: 0});
                        entity.activation_time = Some(key);
                        key
                    } 
                };
                self.activation_times.get(key)
            }
        }
    }
}
