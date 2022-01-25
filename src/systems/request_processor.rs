use super::super::components::*;
use super::super::requests::*;
use lame_ecs::{Entity, World};
use rocket::tokio::sync::mpsc::{error::TryRecvError, UnboundedReceiver};
use rocket::tokio::sync::oneshot::Sender;

pub fn process(world: &mut World, rx: &mut UnboundedReceiver<Request>) -> Result<(), String> {
    let input = rx.try_recv();
    let request = match input {
        Ok(request) => request,
        Err(TryRecvError::Empty) => {
            return Ok(());
        }
        Err(TryRecvError::Disconnected) => {
            return Err("Producer thread diconnected".to_owned());
        }
    };
    match request {
        Request::NewTask(data) => {
            let entity = create_lcn_task(world, data.1);
            send_response(data.0, Response::NewTask(entity), "NewTask");
        }
        Request::RemoveTask(data) => {
            let mut removed = false;
            if world.is_alive(data.1) {
                world.remove_entity(data.1);
                removed = true;
            }
            send_response(data.0, Response::RemoveTask(removed), "RemoveTask");
        }
        Request::GetStatus(tx) => {
            let status = super::status_reporter::get_status(world);
            send_response(tx, Response::GetStatus(status), "GetStatus");
        }
    }
    Ok(())
}

fn send_response(tx: Sender<Response>, response: Response, tag: &str) {
    let result = tx.send(response);
    result.unwrap_or_else(|_| panic!("process_request({}): failed to send response", tag));
}

fn create_lcn_task(world: &mut World, task: TaskRequest) -> Entity {
    println!("new lcn task {}", serde_json::to_string(&task).unwrap());
    let entity = world.new_entity();
    world.add_component(entity, task.schedule);
    world.add_component(entity, ActivationState::ToBeScheduled);
    world.add_component(entity, task.cmd);
    entity
}
