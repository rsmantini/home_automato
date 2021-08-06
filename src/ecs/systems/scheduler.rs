use super::super::world;
use super::super::components;
use chrono;

#[derive(Default)]
pub struct Scheduler {
    pub foo: i32
}

impl world::System for Scheduler {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn process(&mut self) {
        self.foo = 10;
        println!("{:?}", chrono::offset::Utc::now());
    }
} 