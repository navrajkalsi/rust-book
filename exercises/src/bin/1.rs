use std::{collections::HashMap, io};

fn main() {
    // array of ints
    // let list: [i32; 5] = [1, 2, 3, 4, 5];

    let mut list: Vec<i32> = vec![];
    let mut input = String::new();

    println!("Please enters the numbers to start\nTo stop, press enter");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to readline");

        match input.trim().parse() {
            Ok(num) => {
                list.push(num);
            }
            Err(_) => {
                println!("Stopping");
                break;
            }
        }
        input.clear(); // readline only appends, does not overwrite
    }

    if list.is_empty() {
        println!("No input");
        return;
    }

    list.sort();
    let len = list.len();
    println!("Sorted Input: {list:?}, of len: {len}");
    println!("Median: {}", list[len / 2]);

    let mut map = HashMap::new();
    for num in list {
        *map.entry(num).or_insert(0) += 1;
    }
    // println!("Map: {map:?}");
    let mut mode: (Option<i32>, i32) = (None, 0);
    for (key, value) in map {
        mode = match mode.0 {
            Some(_) => {
                if value > mode.1 {
                    (Some(key), value)
                } else {
                    mode
                }
            }
            None => (Some(key), value),
        }
    }
    if let Some(i) = mode.0 {
        println!("Mode: {i}, repeated {} times", mode.1);
    }
}
