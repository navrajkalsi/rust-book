#[derive(Debug)]
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        // consumes self(DraftPost)
        PendingReviewPost {
            content: self.content,
            // content from DraftPost is moved
            approve: false,
        }
    }
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
    approve: bool,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve_double(mut self) -> Result<Post, Self> {
        if self.approve {
            Ok(Post {
                content: self.content,
            })
        } else {
            self.approve = true;
            Err(self)
        }
    }
}
