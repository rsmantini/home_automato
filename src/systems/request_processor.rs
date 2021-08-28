use super::super::requests::*;
use ecs::components::ActivationState;
use ecs::world::{Entity, World};
use std::sync::mpsc;

pub fn process(world: &mut World, rx: &mpsc::Receiver<Request>) -> bool {
    let input = rx.try_recv();
    let request = match input {
        Ok(request) => request,
        Err(mpsc::TryRecvError::Empty) => {
            return true;
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            return false;
        }
    };
    match request {
        Request::NewTask(data) => {
            let entity = create_lcn_task(world, data.1);
            let result = data.0.send(Response::NewTask(entity));
            result.expect("process_request(NewTask): failed to send response");
        }
        Request::GetStatus(tx) => {
            let status = super::status_reporter::get_status(world);
            let result = tx.send(Response::GetStatus(status));
            result.expect("process_request(GetStatus): failed to send response");
        }
    }
    true
}

fn create_lcn_task(world: &mut World, task: TaskRequest) -> Entity {
    println!("new lcn task {}", serde_json::to_string(&task).unwrap());
    let entity = world.new_entity();
    world.add_component(entity, task.schedule);
    world.add_component(entity, ActivationState::ToBeScheduled);
    world.add_component(entity, task.cmd);
    entity
}