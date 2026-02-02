// these tests are meant to test the entire public facing functionality of a library

use testing::add_two;

// putting common into its separate module does not compile the modules code on each cargo test run
mod common;

// no need for [cfg(test)] because cargo treat tests dir specially and compiles them only when
// using cargo test
#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
