use chrono::{Datelike, Local, TimeZone};
use ecs::components::ActivationState;
use ecs::world::World;

pub fn process(world: &mut World) {
    let now = chrono::Local::now();
    process_internal(world, &now);
}

fn process_internal(world: &mut World, now: &chrono::DateTime<chrono::Local>) {
    let range = itertools::izip!(
        &mut world.components.activation_states,
        &world.components.schedules,
        &world.entities
    )
    .filter_map(|(s, a, e)| Some((s.as_mut()?, a.as_ref()?, e)));

    let mut to_be_removed: Vec<ecs::world::Entity> = Vec::new();
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
        let today = now.weekday().num_days_from_monday() as usize;
        if activation_date >= *now && (!has_repeat(&schedule.weekdays) || schedule.weekdays[today]) {
            *state = ActivationState::Scheduled(activation_date.timestamp());
            println!(
                "Entity {} scheduled0 {}",
                entity.id(),
                activation_date.to_rfc2822()
            );
            continue;
        }
        let days = days_to_next_run(now.weekday().num_days_from_monday(), &schedule.weekdays);
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

fn has_repeat(weekdays: &[bool]) -> bool {
    for day in weekdays {
        if *day {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Timelike};
    use ecs::components::*;
    use ecs::world::{Entity, World};

    fn to_schedule(date_time: chrono::DateTime<chrono::Local>) -> Schedule {
        Schedule {
            hour: date_time.hour() as i8,
            min: date_time.minute() as i8,
            sec: date_time.second() as i8,
            weekdays: [false; 7],
        }
    }

    fn new_action(world: &mut World, schedule: Schedule) -> Entity {
        let entity = world.new_entity();
        world.add_component(entity, schedule);
        world.add_component(entity, ActivationState::ToBeScheduled);
        entity
    }

    #[test]
    fn test_one_time_action() {
        let mut world = World::new();
        let mut now = chrono::Local::now();
        if now.hour() >= 23 {
            now = now - Duration::hours(1);
        }
        let action = new_action(&mut world, to_schedule(now + Duration::hours(1)));

        process_internal(&mut world, &now);
        let action_state = world.get_component::<ActivationState>(action).unwrap();
        now = now + Duration::hours(1);
        assert_eq!(*action_state, ActivationState::Scheduled(now.timestamp()));

        process_internal(&mut world, &now);
        let action_state = world.get_component::<ActivationState>(action).unwrap();
        assert_eq!(*action_state, ActivationState::ReadyToRun);

        *action_state = ActivationState::ToBeScheduled;
        process_internal(&mut world, &now);
        assert!(!world.is_alive(action));

        let action = new_action(&mut world, to_schedule(now - Duration::hours(1)));
        assert!(world.is_alive(action));
        process_internal(&mut world, &now);
        assert!(!world.is_alive(action));
    }

    #[test]
    fn test_one_time_action_in_the_past() {
        let mut world = World::new();
        let mut now = chrono::Local::now();
        if now.hour() < 1 {
            now = now + Duration::hours(1);
        }

        let action = new_action(&mut world, to_schedule(now - Duration::hours(1)));
        assert!(world.is_alive(action));
        process_internal(&mut world, &now);
        assert!(!world.is_alive(action));
    }

    #[test]
    fn test_everyday_action() {
        let mut world = World::new();
        let mut now = chrono::Local::now();
        let mut scheduled_time = now - Duration::hours(1);

        let mut schedule = to_schedule(scheduled_time.clone());
        schedule.weekdays = [true; 7];
        let action = new_action(&mut world, schedule);

        let mut repeat = 0;
        while repeat < 8 {
            process_internal(&mut world, &now);

            scheduled_time = scheduled_time + Duration::days(1);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                assert_eq!(
                    *action_state,
                    ActivationState::Scheduled(scheduled_time.timestamp())
                );
            }

            now = now + Duration::days(1);
            process_internal(&mut world, &now);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                assert_eq!(*action_state, ActivationState::ReadyToRun);
                *action_state = ActivationState::ToBeScheduled;
            }
            repeat += 1;
        }
    }

    #[test]
    fn test_mult_day_action() {
        let mut world = World::new();
        let mut now = chrono::Local::now();
        let mut schedule = to_schedule(now.clone());

        let today = now.weekday().num_days_from_monday() as usize;
        schedule.weekdays[(today + 2) % 7] = true;
        schedule.weekdays[(today + 6) % 7] = true;

        println!("days {:?}", schedule.weekdays);
        let action = new_action(&mut world, schedule);

        process_internal(&mut world, &now);

        {
            let action_state = world.get_component::<ActivationState>(action).unwrap();
            let scheduled_time = now + Duration::days(2);
            assert_eq!(
                *action_state,
                ActivationState::Scheduled(scheduled_time.timestamp())
            );
        }

        now = now + Duration::days(2);
        process_internal(&mut world, &now);

        {
            let action_state = world.get_component::<ActivationState>(action).unwrap();
            assert_eq!(*action_state, ActivationState::ReadyToRun);
            *action_state = ActivationState::ToBeScheduled;
        }

        let mut repeat = 0;
        while repeat < 2 {
            process_internal(&mut world, &now);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                let scheduled_time = now + Duration::days(4);
                assert_eq!(
                    *action_state,
                    ActivationState::Scheduled(scheduled_time.timestamp())
                );
            }

            now = now + Duration::days(4);
            process_internal(&mut world, &now);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                assert_eq!(*action_state, ActivationState::ReadyToRun);
                *action_state = ActivationState::ToBeScheduled;
            }

            process_internal(&mut world, &now);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                let scheduled_time = now + Duration::days(3);
                assert_eq!(
                    *action_state,
                    ActivationState::Scheduled(scheduled_time.timestamp())
                );
            }

            now = now + Duration::days(3);
            process_internal(&mut world, &now);

            {
                let action_state = world.get_component::<ActivationState>(action).unwrap();
                assert_eq!(*action_state, ActivationState::ReadyToRun);
                *action_state = ActivationState::ToBeScheduled;
            }
            repeat += 1;
        }
    }

    #[test]
    fn add_action_for_next_day_after_current_time() {
        let mut world = World::new();
        let now = chrono::Local::now();

        let mut schedule = to_schedule(now + Duration::seconds(1));

        let today = now.weekday().num_days_from_monday() as usize;
        let tomorrow = (today + 1) % 7;
        schedule.weekdays[tomorrow] = true;

        let action = new_action(&mut world, schedule);
        process_internal(&mut world, &now);
        let state = world.get_component::<ActivationState>(action).unwrap();
        let expected_sched_time = now + Duration::seconds(1) + Duration::days(1);
        assert_eq!(
            *state,
            ActivationState::Scheduled(expected_sched_time.timestamp())
        );
    }
}
