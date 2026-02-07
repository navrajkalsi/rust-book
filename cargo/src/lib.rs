//! # Cargo
//!
//! this syntax is used to document the item that contains the comment
//! this comment describes this entire library crate, as it is for the crate root

// the following is a documentation code, using markdown format
// use cargo doc --open to view the html
// the code used for examples in documentation is also run on cargo test as tests

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
