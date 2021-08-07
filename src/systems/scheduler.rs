use ecs::world::World;
/*use super::super::components::schedule::*;
use super::super::components::activation_time::*;
use chrono;*/

#[derive(Default)]
pub struct Scheduler {}

impl Scheduler {
    pub fn process(&self, world: &mut World) {
        std::thread::sleep_ms(1000);
        let t = world
            .components
            .schedules
            .iter_mut()
            .zip(world.components.activation_times.iter_mut())
            .filter_map(|(s, a)| Some((s.as_mut()?, a.as_mut()?)));
        for (schedule, activation_time) in t {
            println!(
                "min: {} at: {}",
                schedule.hour, activation_time.seconds_to_acivate
            );
        }
        println!("-------------");
        //let entity = Entity{id: 0};
        //let s = world.get_component::<Schedule>(entity);
        //let a = world.get_component::<ActivationTime>(entity);
        //if !s.is_none() && !a.is_none() {

        //}
        //println!("{:?}", chrono::offset::Utc::now());
    }
}
