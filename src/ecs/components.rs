pub mod activation_state;
pub mod schedule;

pub use activation_state::ActivationState;
pub use schedule::Schedule;

#[derive(Default)]
pub struct Components {
    pub schedules: Vec<Option<Schedule>>,
    pub activation_states: Vec<Option<ActivationState>>,
}

impl ComponentVec for ActivationState {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>> {
        &mut c.activation_states
    }
}

impl ComponentVec for Schedule {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>> {
        &mut c.schedules
    }
}

impl Components {
    pub fn push(&mut self) {
        self.activation_states.push(None);
        self.schedules.push(None);
    }

    pub fn remove(&mut self, index: usize) {
        self.activation_states.remove(index);
        self.schedules.remove(index);
    }
}

pub trait ComponentVec: Sized {
    fn get_vec(c: &mut Components) -> &mut Vec<Option<Self>>;
}
