fn main() {
    let mut _s = String::new();

    let data = "initial contents";

    // to_string is available for any type that implements the display trait
    let _s = data.to_string();

    // The method also works on a literal directly:
    let _s = "initial contents".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // s2 is still usable, cause push_str just requires a reference
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // reference for s2 is required because of the definition of the function

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // s1 is moved
    let _s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // now ownership transfer occurs with format macro
    let _s = format!("{s1}-{s2}-{s3}");
}
