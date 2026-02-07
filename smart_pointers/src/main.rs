use smart_pointers::{MyBox, hello};
use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x; // referencing data on stack

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // referencing data on heap, which is tied to this pointer
    // here x is copied on the heap and then the address is used as y
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // untill now a reference and box have been behaving the same way

    // CHECK lib.rs
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // dereferencing a custom type requires implementing the deref trait
    assert_eq!(*(y.deref()), 5); // behind the scenes this is what rust does with deref

    let name = MyBox::new(String::from("Navraj"));

    // without deref coercion:
    hello(&(*name)[..]);

    // with deref coercion:
    // rust automatically derefs the boxed value to get a String
    // and then passes a reference to String to get &str
    // this only happends for types that implement the deref trait
    hello(&name);

    // Converting one mutable reference to one immutable reference will
    // never break the borrowing rules.
    // Converting an immutable reference to a mutable reference would
    // require that the initial immutable reference is the only immutable reference to that data,
    // but the borrowing rules don’t guarantee that.
    // Therefore, Rust can’t make the assumption that converting an immutable reference
    // to a mutable reference is possible.
}
