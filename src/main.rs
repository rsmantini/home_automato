use ecs::components::*;
use rocket::*;
use std::sync::mpsc;

mod systems;

#[get("/<number>")]
fn index(tx: &State<mpsc::SyncSender<i32>>, number: i32) -> String {
    tx.try_send(number).unwrap();
    let s = format!("recived {}", number);
    s
}

fn event_loop(_rx: mpsc::Receiver<i32>) {
    let mut world = ecs::world::World::new();
    let e0 = world.new_entity();
    world.add_component(
        e0,
        Schedule {
            hour: 22,
            min: 12,
            sec: 0,
            weekdays: [true; 7],
        },
    );
    world.add_component(e0, ActivationState::ToBeScheduled);

    let e1 = world.new_entity();
    world.add_component(
        e1,
        Schedule {
            hour: 22,
            min: 45,
            sec: 0,
            weekdays: [false; 7],
        },
    );
    world.add_component(e1, ActivationState::ToBeScheduled);

    let e2 = world.new_entity();
    world.add_component(
        e2,
        Schedule {
            hour: 21,
            min: 30,
            sec: 0,
            weekdays: [false; 7],
        },
    );
    world.add_component(e2, ActivationState::ToBeScheduled);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        //println!("recieved {}", rx.recv().unwrap());
        systems::scheduler::process(&mut world);
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop(rx));
    rocket::build().manage(tx).mount("/", routes![index])
}
