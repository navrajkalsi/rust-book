use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())

        // if let Some(pref) = user_preference {
        //     pref
        // } else {
        //     self.most_stocked()
        // }
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // this is a closure and can be called just like a function
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(2);

    // types for closures are normally infered and do not require annotations (mostly)

    // fn add_one_v1(x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;

    // if no type is specified the closure will use the type with what it is used first
    // and then cannot be used with any other type

    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // this will fail as now the closure is bound to use String
    // let n = example_closure(5);

    // closures take the minimum amount of ownership that is needed to run it
    // increasing order of "ownership": immutable reference/borrowing, mutable reference/borrowing, transfering ownership

    // immutable reference/borrow, can also print between closure definiton and use, as their can
    // be more than one immut references
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    // can print here
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // mutable reference/borrow, cannot use print between closure definiton and use, as their
    // cannot be a immutable references, if another mutable reference already exists
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // cannot call the following line
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    // transfering ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // move can be used to force the closure to take the ownership
    // cannot pass an reference to a new thread as this main thread may finish first and therefore
    // drop the referenced value, which would mean that the reference inside of the new thread
    // would be invalid
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
