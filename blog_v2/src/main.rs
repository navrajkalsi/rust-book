use blog_v2::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    // extras
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post
        .approve_double()
        .expect_err("Must be called again to be a Post type");

    let post = post.approve_double().expect("Return type must of Post");
    assert_eq!("I ate a salad for lunch today", post.content());
}
