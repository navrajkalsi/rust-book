use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // send could fail, if the receiver has already been dropped for example
        tx.send(val).unwrap();
        // send also transfers ownership, therefore we cannot use val now in this thread and it has
        // been moved to the main thread(or more precisely the receiver)
    });

    // recv blocks until something is received from the new thread
    // try_recv is the non-blocking version
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    let (tx, rx) = mpsc::channel();
    // more than one transmitters can transmit to one receiver(only)
    // this is done by clonning the original transmitters
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // here recv is not called, rather rx is treated as an iterator, which will exhaust once the
    // channel is closed in the new thread
    for received in rx {
        println!("Got: {received}");
    }
}
