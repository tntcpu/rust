pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        //1
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        //2
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        //3
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
