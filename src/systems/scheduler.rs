use chrono::{Datelike, Local, TimeZone};
use ecs::components::ActivationState;
use ecs::world::World;

pub fn process(world: &mut World) {
    let range = itertools::izip!(
        &mut world.components.activation_states,
        &world.components.schedules,
        &world.entities
    )
    .filter_map(|(s, a, e)| Some((s.as_mut()?, a.as_ref()?, e)));

    let mut to_be_removed: Vec<ecs::world::Entity> = Vec::new();
    let now = chrono::Local::now();
    for (state, schedule, entity) in range {
        if *state == ActivationState::ReadyToRun {
            continue;
        }
        if let ActivationState::Scheduled(activation_time) = *state {
            if activation_time <= now.timestamp() {
                *state = ActivationState::ReadyToRun;
                println!("Entity {} ready to run", entity.id());
            }
            continue;
        }
        let activation_date = Local.ymd(now.year(), now.month(), now.day()).and_hms(
            schedule.hour as u32,
            schedule.min as u32,
            schedule.sec as u32,
        );
        if activation_date >= now {
            *state = ActivationState::Scheduled(activation_date.timestamp());
            println!(
                "Entity {} scheduled0 {}",
                entity.id(),
                activation_date.to_rfc2822()
            );
            continue;
        }
        let days = days_to_next_run(now.weekday().num_days_from_sunday(), &schedule.week_days);
        if days.is_none() {
            to_be_removed.push(*entity);
            continue;
        }
        let activation_date = activation_date + chrono::Duration::days(days.unwrap());
        *state = ActivationState::Scheduled(activation_date.timestamp());
        println!(
            "Entity {} scheduled1: {}",
            entity.id(),
            activation_date.to_rfc2822()
        );
    }
    for entity in to_be_removed {
        world.remove_entity(entity);
        println!("Entity {} removed", entity.id());
    }
}

fn days_to_next_run(mut weekday: u32, weekdays: &[bool]) -> Option<i64> {
    let mut days = 1;
    while days < weekdays.len() {
        weekday = (weekday + 1) % weekdays.len() as u32;
        if weekdays[weekday as usize] {
            return Some(days as i64);
        }
        days += 1;
    }
    None
}
