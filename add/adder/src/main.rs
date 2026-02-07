fn main() {
    let num = 10;
    // can use add_one because in Cargo.toml for this binary crate, we have mentioned the add_one
    // library crate's path in the dependencies section
    println!("{num} plus one is {}!", add_one::add_one(num));
    println!("{num} plus two is {}!", add_two::add_two(num));
}
