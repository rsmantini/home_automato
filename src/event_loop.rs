use super::components::*;
use super::lcn_config;
use super::requests::*;
use super::systems;
use rocket::tokio::{runtime::Runtime, sync::mpsc::UnboundedReceiver};

pub fn run(mut rx: UnboundedReceiver<Request>) -> Result<(), String> {
    let mut world = lame_ecs::create_world!();
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(1))
        .build()
        .expect("could not init http client");
    let lcn_config = lcn_config::from_file("lcn_config.json").expect("could not parse config file");
    let runtime = Runtime::new().expect("could not create tokio runtime");
    loop {
        runtime.block_on(systems::request_processor::process(&mut world, &mut rx))?;
        systems::scheduler::process(&mut world);
        systems::lcn_command_executor::process(&mut world, &lcn_config, &client);
    }
}
