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
    let s = systems::Scheduler::default();
    let e0 = world.new_entity();
    world.add_component(
        e0,
        Schedule {
            hour: 1,
            min: 0,
            sec: 0,
            repeat: false,
        },
    );
    world.add_component(
        e0,
        ActivationTime {
            seconds_to_acivate: 10,
        },
    );

    let e1 = world.new_entity();
    world.add_component(
        e1,
        Schedule {
            hour: 2,
            min: 0,
            sec: 0,
            repeat: false,
        },
    );

    let e2 = world.new_entity();
    world.add_component(
        e2,
        Schedule {
            hour: 3,
            min: 0,
            sec: 0,
            repeat: false,
        },
    );
    world.add_component(
        e2,
        ActivationTime {
            seconds_to_acivate: 30,
        },
    );

    loop {
        //println!("recieved {}", rx.recv().unwrap());
        s.process(&mut world);
    }
}

#[launch]
fn rocket() -> _ {
    let (tx, rx) = mpsc::sync_channel(100);
    std::thread::spawn(move || event_loop(rx));
    rocket::build().manage(tx).mount("/", routes![index])
}
