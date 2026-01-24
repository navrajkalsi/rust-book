fn loop_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break can also return expressions to the outside of the loop
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    for number in (1..4).rev() {
        // rev() reverses the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    loop_loop();

    while_loop();

    for_loop();
}
