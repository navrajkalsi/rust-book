#![allow(unused)]

use std::fmt::Display;

fn main() {
    let string1 = String::from("long string");

    {
        // smaller lifetime begins
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    } // smaller lifetime ends, string 2 and result cannot be used, string1 can be used

    let result; // moving result to larger lifetime
    {
        // smaller lifetime begins
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } // smaller lifetime ends, string 2 is out of scope, but result was declared in outer scope,
    // but since lifetime of result reference will be the lifetime of smaller of the two strings,
    // with string2, result is also now not valid

    // println!("The longest string is {result}"); // will cause errors

    // all string literals use static lifetimes that are valid for the whole program
    let s: &'static str = "I have a static lifetime.";

    let string2 = "xyz";
    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {result}");
}

// just like generic types, have to specify what is 'a inside of angle brackets before use inside
// funciton parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a will the smaller of lifetimes of x and y vars
    // for some lifetime 'a, the function takes two parameters,
    // both of which are string slices that live at least as long as lifetime 'a.
    // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a
    if x.len() > y.len() { x } else { y }
}

// this function also returns and accepts a reference, but DOES NOT require use to annotate the
// lifetimes explicitly.
// this is because in this case the lifetime is inferred and is implicit
// we could have written:
// fn first_word<'a>(s: &'a str) -> &'a str {
// these are called lifetime elision rules and just happens for a few deterministic patterns
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.

// this function uses generic datatype(T), genereic lifetime('a), trait bound(T: Display) specified
// inside the where clause
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
