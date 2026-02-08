use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let m = Mutex::new(5);

    {
        // lock aquires the lock
        // NOTE this blocks the current thread until this thread has the lock
        // though this is a single threaded program right now
        // calling lock could PANIC if another thread holding the lock panicked.
        // In that case, no one would ever be able to get the lock,
        // so we’ve chosen to unwrap and have this thread panic if we’re in that situation.
        let mut num = m.lock().unwrap();
        *num = 6;
        // here num is a MutexGuard, which implements the Deref and Drop traits
        // we can deref MutexGuard to get the inner value(i32 in this case)
        // the drop call will release the mutex lock automatically at the end of scope
    }

    println!("m = {m:?}");

    // SHARED MUTEX AMONG MULTIPLE THREADS
    // Rc makes it possible for multiple threads to access the counter mutex
    // Arc is atomic version of Rc, which is thread safe
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // this counter is smart pointer reference counter that can be moved to the new thread
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // though counter itself is not mutable,
            // it can provide mutable reference to the underlying data
            //
            // cannot transfer ownership of counter to new thread
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
