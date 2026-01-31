// see lib.rs
use traits::{SocialPost, Summary};

fn main() {
    println!("Hello, world!");
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
