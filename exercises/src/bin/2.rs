use std::io;

fn main() {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut input = String::new();

    println!("Provide an english word");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to readline");

    for c in input.trim().chars() {
        if c.is_whitespace() {
            println!("Only one word is supported");
            return;
        }
    }

    let input = input.trim();
    let mut chars = input.chars();
    if let Some(first) = chars.next() {
        if vowels.contains(&first) {
            println!("{input}-hay");
        } else {
            for single in chars {
                print!("{single}");
            }
            println!("-{first}ay");
        }
    }
}
