use requests::*;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, State};
use std::sync::mpsc;

mod components;
mod event_loop;
mod lcn_config;
mod requests;
mod systems;

#[post("/new_lcn_task", data = "<task>")]
fn lcn_task_producer(
    global_tx: &State<mpsc::SyncSender<Request>>,
    task: Json<TaskRequest>,
) -> String {
    let (tx, rx) = mpsc::sync_channel(1);
    let request = Request::NewTask((tx, task.into_inner()));
    let response = make_request(global_tx, rx, request);
    match response {
        Ok(Response::NewTask(entity)) => {
            let res = format!("success: task created with with id {}", entity.id());
            serde_json::to_string(&res).unwrap()
        }
        Ok(_) => serde_json::to_string("failure: unexpected response").unwrap(),
        Err(e) => serde_json::to_string(&e.to_string()).unwrap(),
    }
}

#[get("/remove_task/<id>")]
fn remove_task(global_tx: &State<mpsc::SyncSender<Request>>, id: i64) -> String {
    let (tx, rx) = mpsc::sync_channel(1);
    let request = Request::RemoveTask((tx, ecs::Entity::new(id)));
    let response = make_request(global_tx, rx, request);
    match response {
        Ok(Response::RemoveTask(true)) => {
            let res = format!("success: task with id {} removed", id);
            serde_json::to_string(&res).unwrap()
        }
        Ok(Response::RemoveTask(false)) => {
            let res = format!("failure: no task with id {} exists", id);
            serde_json::to_string(&res).unwrap()
        }
        Ok(_) => serde_json::to_string("failure: unexpected response").unwrap(),
        Err(e) => serde_json::to_string(&e.to_string()).unwrap(),
    }
}

#[get("/get_status")]
fn get_status(global_tx: &State<mpsc::SyncSender<Request>>) -> String {
    let (tx, rx) = mpsc::sync_channel(1);
    let response = make_request(global_tx, rx, Request::GetStatus(tx));
    match response {
        Ok(Response::GetStatus(status)) => serde_json::to_string(&status).unwrap(),
        Ok(_) => serde_json::to_string("failure: unexpected response").unwrap(),
        Err(e) => serde_json::to_string(&e.to_string()).unwrap(),
    }
}

#[get("/")]
fn index() -> rocket_dyn_templates::Template {
    rocket_dyn_templates::Template::render("home", "")
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop::run(rx));
    rocket::build()
        .manage(tx)
        .mount("/", routes![index])
        .mount("/api", routes![lcn_task_producer, remove_task, get_status])
        .attach(rocket_dyn_templates::Template::fairing())
}
