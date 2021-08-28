use super::lcn_config;
use super::requests::*;
use super::systems;

pub fn run(rx: std::sync::mpsc::Receiver<Request>) {
    let mut world = ecs::world::World::new();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .expect("could not init http client");
    let lcn_config = lcn_config::from_file("lcn_config.json").expect("could not parse config file");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !systems::request_processor::process(&mut world, &rx) {
            break;
        }
        systems::scheduler::process(&mut world);
        systems::lcn_command_executor::process(&mut world, &lcn_config, &client);
    }
}