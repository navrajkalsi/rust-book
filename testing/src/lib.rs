// pass -- --test-thread=1 to disable parallelism so that all tests run in a single thread
// this could be helpful if the tests depend on eachother
//
// pass -- --show-output to enable printing stdout for tests that pass
// by default you only see the stdout of failing test, even incase of an explicit println call
//
// pass the name of a function to test just that particular function
// to match multiple tests pass the common part of the function names
//
// pass -- --ignored to test functions that have the #[ignored] attribute before their definitions
#![allow(unused)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// assert demo
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// assert_eq demo
pub fn add_two(num: u64) -> u64 {
    num + 2
}

// assert format print on fail
fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

// should_panic test
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // should_panic expected check
    pub fn new_split(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}

// this attribute tells cargo to only compile the tests when cargo test is run
// cfg stands for configuration and tells cargo to only run the code when a specific config option
// is supplied, which in this case in (test)
//
// this module contains unit tests, tests that test certain functionality of a library from within
// the library that may or may not be public facing
#[cfg(test)]
mod tests {
    // imports rectangle from parent 'testing' module
    use super::*;

    // attribute test so that the test runner knows to treat this function as a test and run it on
    // cargo test
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed
    #[test]
    fn another() {
        return;
        panic!("Deliberate fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // using assert for always true results
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // smaller should not be able to hold larger
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert also takes in optional formated string for debugging
        // everything after the required arguments is passed onto the format! macro that prints on
        // test fail
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // adding should_panic ensures the function always panics in order to mark the test as pass
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // here we pass a substring to expected so that we can be sure the panic happened because of
    // the line that we want to test and not some other bug that we do not know about right now
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_split() {
        Guess::new_split(200);
    }

    #[test]
    #[ignore]
    // can only be tested by providing the -- --ignored flag to cargo test
    fn expensive_test() {
        ()
    }
}
