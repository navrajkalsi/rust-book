// before re-exporting the types:
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// after re-exporting in lib.rs:
use navrajkalsi_art::{PrimaryColor, mix};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
