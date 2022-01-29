use super::components::*;
use super::lcn;
use super::requests::*;
use super::systems;
use rocket::tokio::{runtime::Runtime, sync::mpsc::UnboundedReceiver};

pub fn run(mut rx: UnboundedReceiver<Request>) -> Result<(), String> {
    let mut world = lame_ecs::create_world!();
    let runtime = Runtime::new().expect("could not create tokio runtime");
    std::thread::sleep(std::time::Duration::from_secs(1));
    let lcn_client = lcn::build_lcn_client().expect("could not build lcn client");
    loop {
        runtime.block_on(systems::request_processor::process(&mut world, &mut rx))?;
        systems::scheduler::process(&mut world);
        systems::lcn_command_executor::process(&mut world, &lcn_client);
    }
}
