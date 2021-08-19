use ecs::components::*;
use ecs::world::{Entity, World};
use rocket::*;
use std::sync::mpsc;

mod lcn_config;
mod systems;

#[get("/<number>")]
fn index(tx: &State<mpsc::SyncSender<i32>>, number: i32) -> String {
    tx.try_send(number).unwrap();
    let s = format!("recived {}", number);
    s
}

fn new_lcn_command(world: &mut World) -> Entity {
    let entity = world.new_entity();
    let schedule = Schedule {
        hour: 23,
        min: 34,
        sec: 0,
        weekdays: [false; 7],
    };
    world.add_component(entity, schedule);
    world.add_component(entity, ActivationState::ToBeScheduled);
    world.add_component(entity, LcnCommand { button_id: 1623 });
    entity
}

fn event_loop(_rx: mpsc::Receiver<i32>) {
    let mut world = ecs::world::World::new();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .expect("could not init http client");
    let lcn_config = lcn_config::from_file("lcn_config.json").expect("could not parse config file");
    new_lcn_command(&mut world);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        //println!("recieved {}", rx.recv().unwrap());
        systems::scheduler::process(&mut world);
        systems::lcn_command_executor::process(&mut world, &lcn_config, &client);
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop(rx));
    rocket::build().manage(tx).mount("/", routes![index])
}
