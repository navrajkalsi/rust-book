use smart_pointers::{CustomSmartPointer, MyBox, hello};
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

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
    // now the drop method will be called we implementated the drop trait on the struct
    // the vars are dropped in reverse order of their declarations
    // values can also be cleaned manually
    // NOTE: this is not us calling the drop method, this is a different function from
    // std::mem::drop
    // the method drop cannot be called explicitly because of double free error as rust will also
    // try to call it regardless at the end of the scope
    drop(_d);
    println!("CustomSmartPointer dropped before the end of main");

    // NOTE:
    // Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    //
    // Box<T> allows immutable or mutable borrows checked at compile time;
    // Rc<T> allows only immutable borrows checked at compile time;
    // RefCell<T> allows immutable or mutable borrows checked at runtime.
    //
    // Because RefCell<T> allows mutable borrows checked at runtime,
    // you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
}
