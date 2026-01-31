#![allow(unused)]

// accepts a slice of i32s and returns the largest from those
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// accepts a slice of chars and returns the largest from those
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// T has to be defined before use inisde the function parameters.
// that is done inside <>
// here T also has to have the PartialOrd trait defined on it for use to compare it with binary
// operators
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// can also be used for structs
struct Point<T, U> {
    x: T,
    y: U,
}

// also for method definitions
// again we have to declare just after impl that T & U are generics, with <>
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// also for enums
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // functions
    let number_list = vec![34, 50, 25, 100, 65];

    let mut result = largest_i32(&number_list);
    println!("The largest number is {result}");
    assert_eq!(*result, 100);
    result = largest(&number_list);
    println!("The largest number is {result}");
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let mut result = largest_char(&char_list);
    println!("The largest char is {result}");
    assert_eq!(*result, 'y');
    result = largest_char(&char_list);
    println!("The largest char is {result}");
    assert_eq!(*result, 'y');

    // structs
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    // method definitions
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
