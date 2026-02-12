use blog::Post;

fn main() {
    // original
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // extra functionality
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!(true, post.is_draft());

    post.request_review();
    assert_eq!(true, post.is_pending_review());

    // 1. reject() implementation
    post.reject();
    assert_eq!(true, post.is_draft());

    // 2. approve_double() to take 2 calls to approve
    post.request_review();
    post.approve_double();
    assert_eq!(false, post.is_published());
    post.approve_double();
    assert_eq!(true, post.is_published());

    // 3. add content only in draft state
    let mut post = Post::new();
    post.add_text("original");
    post.request_review();
    post.approve();
    assert_eq!("original", post.content());
    assert_eq!(true, post.is_published());

    post.add_text("new");
    assert_eq!("original", post.content());
}
