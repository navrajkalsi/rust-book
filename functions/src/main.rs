fn five() -> i32 {
    5 // legal, means this is an expresssion(without a semicolon) and will be returned
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

// non returning functions return unit type (()), an empty tuple
