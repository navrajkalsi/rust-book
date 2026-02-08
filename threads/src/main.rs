use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        // closure for the new thread to run
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // here we are blocking the main thread, to wait for the new thread to finish
    // NOTE: try commenting the line and running the code, the new thread terminates early
    // because the main thread will do so after this point(if the next line is commented)
    handle.join().unwrap();

    // using move to transfer ownership to new thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // without move, we would get an error because rust will not be able to guarantee that the
        // reference to v will be valid for the lifetime of this new thread
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
