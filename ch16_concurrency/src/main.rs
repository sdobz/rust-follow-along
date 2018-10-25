use std::thread;
use std::time::Duration;

mod messages;
mod shared_state;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("From thread: {:?}", v);

        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    messages::demo();
    shared_state::demo();
}
