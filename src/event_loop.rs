use super::components;
use super::lcn_config;
use super::requests::*;
use super::systems;

pub fn run(rx: std::sync::mpsc::Receiver<Request>) {
    let components = Box::new(components::Components::default());
    let mut ecs = lame_ecs::Ecs::new(components);
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .expect("could not init http client");
    let lcn_config = lcn_config::from_file("lcn_config.json").expect("could not parse config file");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !systems::request_processor::process(&mut ecs, &rx) {
            break;
        }
        systems::scheduler::process(&mut ecs);
        systems::lcn_command_executor::process(&mut ecs, &lcn_config, &client);
    }
}
