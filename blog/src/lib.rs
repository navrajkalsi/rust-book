pub struct Post {
    state: Option<Box<dyn State>>, // state is an option with some variant that contains a smart
    // pointer to a State trait object
    content: String,
    approve_on_next: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // a new post always starts as a draft
            // because the state field of Post is private,
            // there is no way to create a Post in any other state!
            // In the Post::new function, we set the content field to a new, empty String.
            state: Some(Box::new(Draft {})),
            content: String::from(""),
            approve_on_next: false,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // self.content.push_str(text);
        self.content = self
            .state
            .as_ref()
            .unwrap()
            .add_text(self.content.clone(), text);
    }

    // these methods are the public facing api methods
    // and they call the respective state implementations
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // now self.state would be None
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn approve_double(&mut self) {
        if self.approve_on_next {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        } else {
            self.approve_on_next = true
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn is_draft(&self) -> bool {
        if let Some(s) = self.state.as_ref() {
            s.is_draft()
        } else {
            false
        }
    }

    pub fn is_pending_review(&self) -> bool {
        if let Some(s) = self.state.as_ref() {
            s.is_pending_review()
        } else {
            false
        }
    }

    pub fn is_published(&self) -> bool {
        if let Some(s) = self.state.as_ref() {
            s.is_published()
        } else {
            false
        }
    }
}

trait State {
    // NOTE rather than having self, &self, or &mut self as the first parameter of the method,
    // we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    // default implementation for Draft and PendingReview
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn is_draft(&self) -> bool {
        false
    }

    fn is_pending_review(&self) -> bool {
        false
    }

    fn is_published(&self) -> bool {
        false
    }

    fn add_text(&self, mut content: String, to_add: &str) -> String {
        content
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // cannot jump directly to Published
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_draft(&self) -> bool {
        true
    }

    fn add_text(&self, mut content: String, to_add: &str) -> String {
        content.push_str(to_add);
        content
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // already in PendingReview state, should stay as such
        // we can be certain of this because the parameter specifies that self
        // should be a Box<PendingReview>
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn is_pending_review(&self) -> bool {
        true
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_published(&self) -> bool {
        true
    }
}
