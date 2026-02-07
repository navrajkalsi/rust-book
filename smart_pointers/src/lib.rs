// Note:
// There’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>:
// Our version will not store its data on the heap.
// We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.

use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    // this Target type needs to be associated to our type because the deref method signature
    // requires that
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // needs to return a reference for the * operator to dereference
    }
}

pub fn hello(name: &str) {
    println!("Hello, {name}.");
}
