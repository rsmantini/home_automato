use super::super::components::ActivationState;
use super::super::requests::*;
use ecs::{Ecs, Entity};
use std::sync::mpsc;

pub fn process(ecs: &mut Ecs, rx: &mpsc::Receiver<Request>) -> bool {
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
            let entity = create_lcn_task(ecs, data.1);
            send_response(data.0, Response::NewTask(entity), "NewTask");
        }
        Request::RemoveTask(data) => {
            ecs.remove_entity(data.1);
            send_response(data.0, Response::RemoveTask, "RemoveTask");
        }
        Request::GetStatus(tx) => {
            let status = super::status_reporter::get_status(ecs);
            send_response(tx, Response::GetStatus(status), "GetStatus");
        }
    }
    true
}

fn send_response(tx: mpsc::SyncSender<Response>, response: Response, tag: &str) {
    let result = tx.send(response);
    result.unwrap_or_else(|_| panic!("process_request({}): failed to send response", tag));
}

fn create_lcn_task(ecs: &mut Ecs, task: TaskRequest) -> Entity {
    println!("new lcn task {}", serde_json::to_string(&task).unwrap());
    let entity = ecs.new_entity();
    ecs.add_component(entity, task.schedule);
    ecs.add_component(entity, ActivationState::ToBeScheduled);
    ecs.add_component(entity, task.cmd);
    entity
}
