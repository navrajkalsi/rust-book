#![allow(unused)]
use std::fmt::{Debug, Display};

// One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate
// we can’t implement external traits on external types.
// For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate
//
// just like interfaces
// any type using the Summary trait has to provide its own definition of the methods
pub trait Summary {
    // to have a default definition of summarize, we can create the definition here for every type
    // and then optionally have custom definitions based on every type
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// use for keyword when the method is to be defined for a specific trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// this function accepts any type that implements all the summary trait methods
// this is called 'trait bound'
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// same as the one before, just more verbose
// this is the actual syntax and 'impl Summary' is just syntax sugar
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// use this way to force more than one trait implementations
pub fn notify_extended(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_extended_verbose<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    println!("Verbose");
}

// trait bounds for generics can be moved to 'where'
fn some_function_readable<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("More readable!");
}

// we can also specify traits in the return types
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}
