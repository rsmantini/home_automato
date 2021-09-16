pub use activation_state::ActivationState;
pub use lcn_command::LcnCommand;
pub use schedule::Schedule;

pub mod activation_state;
pub mod lcn_command;
pub mod schedule;

#[derive(Default)]
pub struct Components {
    pub schedules: Vec<Option<Schedule>>,
    pub activation_states: Vec<Option<ActivationState>>,
    pub lcn_commands: Vec<Option<LcnCommand>>,
}

impl lame_ecs::Components for Components {
    fn push_none(&mut self) {
        self.schedules.push(None);
        self.activation_states.push(None);
        self.lcn_commands.push(None);
    }

    fn remove(&mut self, index: usize) {
        self.schedules.remove(index);
        self.activation_states.remove(index);
        self.lcn_commands.remove(index);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl lame_ecs::Component for Schedule {
    fn get_vec(components: &mut dyn lame_ecs::Components) -> &mut Vec<Option<Self>> {
        &mut lame_ecs::downcast_components_mut::<Components>(components).schedules
    }
}

impl lame_ecs::Component for ActivationState {
    fn get_vec(components: &mut dyn lame_ecs::Components) -> &mut Vec<Option<Self>> {
        &mut lame_ecs::downcast_components_mut::<Components>(components).activation_states
    }
}

impl lame_ecs::Component for LcnCommand {
    fn get_vec(components: &mut dyn lame_ecs::Components) -> &mut Vec<Option<Self>> {
        &mut lame_ecs::downcast_components_mut::<Components>(components).lcn_commands
    }
}
