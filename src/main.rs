use ::serde::{Deserialize, Serialize};
use ecs::components::*;
use ecs::world::{Entity, World};
use rocket::serde::json::Json;
use rocket::*;
use std::sync::mpsc;

mod lcn_config;
mod systems;

#[derive(Debug, Deserialize, Serialize)]
pub struct LcnTask {
    pub schedule: Schedule,
    pub cmd: LcnCommand,
}

#[post("/lcn_task", data = "<task>")]
fn lcn_task_producer(tx: &State<mpsc::SyncSender<LcnTask>>, task: Json<LcnTask>) -> String {
    let result = tx.try_send(task.into_inner());
    match result {
        Ok(_) => serde_json::to_string("sucess: task added").unwrap(),
        Err(_) => serde_json::to_string("failure: buffer full").unwrap(),
    }
}

#[get("/")]
fn index() -> rocket_dyn_templates::Template {
    rocket_dyn_templates::Template::render("home", "")
}

fn new_lcn_task(world: &mut World, task: LcnTask) -> Entity {
    println!("new lcn task {}", serde_json::to_string(&task).unwrap());
    let entity = world.new_entity();
    world.add_component(entity, task.schedule);
    world.add_component(entity, ActivationState::ToBeScheduled);
    world.add_component(entity, task.cmd);
    entity
}

fn event_loop(rx: mpsc::Receiver<LcnTask>) {
    let mut world = ecs::world::World::new();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .expect("could not init http client");
    let lcn_config = lcn_config::from_file("lcn_config.json").expect("could not parse config file");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let input = rx.try_recv();
        match input {
            Ok(task) => {
                new_lcn_task(&mut world, task);
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
        systems::scheduler::process(&mut world);
        systems::lcn_command_executor::process(&mut world, &lcn_config, &client);
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop(rx));
    rocket::build()
        .manage(tx)
        .mount("/", routes![index])
        .mount("/api", routes![lcn_task_producer])
        .attach(rocket_dyn_templates::Template::fairing())
}
