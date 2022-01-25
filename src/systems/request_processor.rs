use super::super::components::*;
use super::super::requests::*;
use lame_ecs::{Entity, World};
use rocket::tokio::sync::mpsc::UnboundedReceiver;
use rocket::tokio::sync::oneshot::Sender;
use rocket::tokio::time::timeout;

pub async fn process(world: &mut World, rx: &mut UnboundedReceiver<Request>) -> Result<(), String> {
    let seconds_to_next_task = get_seconds_to_next_execution(world);
    let input = match seconds_to_next_task {
        Some(s) => {
            println!("request_processor: blocking for {} seconds", s);
            let r = timeout(std::time::Duration::from_secs(s), rx.recv()).await;
            if r.is_err() {
                return Ok(());
            }
            r.unwrap()
        }
        None => {
            println!("request_processor: blocking indefinitely");
            rx.recv().await
        }
    };
    let request = match input {
        Some(r) => r,
        None => return Err("Producer thread diconnected".to_owned()),
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

fn get_seconds_to_next_execution(world: &World) -> Option<u64> {
    let mut time: Option<i64> = None;
    let now = chrono::Local::now().timestamp();
    for (state, _) in lame_ecs::component_iter!(world, ActivationState) {
        match state {
            ActivationState::Scheduled(t) if t < &time.unwrap_or(i64::MAX) => time = Some(*t),
            ActivationState::ReadyToRun => return Some(0),
            _ => {}
        }
    }
    let seconds = time? - now;
    if seconds < 0 {
        panic!("task should be executed in the past but is not ready to run");
    }
    Some(seconds as u64)
}
