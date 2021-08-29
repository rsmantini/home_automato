use super::super::components::*;
use ecs::Ecs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskStatus {
    pub id: i64,
    pub activation_time: String,
    pub repeat_days: String,
    pub state: String,
    pub cmd_id: i32,
}

pub fn get_status(ecs: &Ecs) -> Vec<TaskStatus> {
    let components = ecs::downcast_components::<Components>(&ecs.components);
    let range = itertools::izip!(
        &components.activation_states,
        &components.schedules,
        &components.lcn_commands,
        &ecs.entities
    )
    .filter_map(|(a, s, l, e)| Some((a.as_ref()?, s.as_ref()?, l.as_ref()?, e)));

    let mut report = Vec::<TaskStatus>::new();
    for (state, schedule, cmd, entity) in range {
        let id = entity.id();
        let activation_time = format!("{:02}:{:02}", schedule.hour, schedule.min);
        let cmd_id = cmd.id;
        let repeat_days = weekdays_to_string(&schedule.weekdays);
        let state = state_to_string(state);
        report.push(TaskStatus {
            id,
            activation_time,
            repeat_days,
            state,
            cmd_id,
        });
    }
    report
}

fn weekdays_to_string(weekdays: &[bool; 7]) -> String {
    let mut result = String::new();
    if weekdays[0] {
        result.push_str("Mon");
    }
    if weekdays[1] {
        result.push_str(" ,Tue");
    }
    if weekdays[2] {
        result.push_str(" ,Wed");
    }
    if weekdays[3] {
        result.push_str(" ,Thu");
    }
    if weekdays[4] {
        result.push_str(" ,Fri");
    }
    if weekdays[5] {
        result.push_str(" ,Sat");
    }
    if weekdays[6] {
        result.push_str(" ,Sun");
    }
    if result.starts_with(" ,") {
        result.remove(0);
        result.remove(0);
    }
    result
}

fn state_to_string(state: &ActivationState) -> String {
    match state {
        ActivationState::ToBeScheduled => String::from("To be scheduled"),
        ActivationState::Scheduled(_) => String::from("Scheduled"),
        ActivationState::ReadyToRun => String::from("Ready to run"),
    }
}
