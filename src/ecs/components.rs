pub mod activation_time;
pub mod schedule;

pub use activation_time::ActivationTime;
pub use schedule::Schedule;

#[derive(Default)]
pub struct Components {
    pub schedules: Vec<Option<Schedule>>,
    pub activation_times: Vec<Option<ActivationTime>>,
}

impl ComponentVec for ActivationTime {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>> {
        &mut c.activation_times
    }
}

impl ComponentVec for Schedule {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>> {
        &mut c.schedules
    }
}

impl Components {
    pub fn push(&mut self) {
        self.activation_times.push(None);
        self.schedules.push(None);
    }

    pub fn remove(&mut self, index: usize) {
        self.activation_times.remove(index);
        self.schedules.remove(index);
    }
}

pub trait ComponentVec: Sized {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>>;
}
