fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // all keys must be of the same type so should be the values, although key and value types may differ

    let team_name = String::from("Blue");
    // copied gives us an Option<i32> from Option<&i32> and then unwrap_or gives use 0 for None
    // could also have used match here
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {score}");

    // can also use in a loop like vectors
    for (key, value) in &scores {
        // the printing order is arbitrary
        println!("{key}: {value}");
    }

    // types implementing the copy trait are copied into the hash map, like i32
    // other types, like String, are moved

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and

    // this overwrites
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    scores.clear();
    scores.insert(String::from("Blue"), 10);

    // do not replace, just insert if key not found
    // entry() returns an enum Entry,
    // the return value is a mutable reference to the value(old or inserted)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // update based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // count is mutable reference to word (old or newly inserted)
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
