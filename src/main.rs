use rocket::*;
use std::sync::mpsc;
use ecs;

#[get("/<number>")]
fn index(tx: &State<mpsc::SyncSender<i32>>, number: i32) -> String {
    tx.try_send(number).unwrap();
    let s = format!("recived {}", number);
    s
}

fn event_loop(rx: mpsc::Receiver<i32>) {
    let mut world = ecs::world::World::new();
    ecs::systems::scheduler::foo();
    loop {
        //println!("recieved {}", rx.recv().unwrap());
        world.update();
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop(rx));
    rocket::build().manage(tx).mount("/", routes![index])
}