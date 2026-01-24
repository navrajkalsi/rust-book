fn main() {
    let number = 3;

    if number < 5 {
        // true arm
        println!("condition was true");
    } else {
        // false arm
        println!("condition was false");
    }

    expression_if();
}

fn expression_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // if can also be used as an expression
    //
    // both arms in if should return the same data type
    // or the compiler cannot infer the data type to use for var number

    println!("The value of number is: {number}");
}
