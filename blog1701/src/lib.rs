pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), //1
            content: String::new(),          //2
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // ""
        self.state.as_ref().unwrap().content(&self)
    }

    //1
    pub fn request_review(&mut self) {
        //2
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()) //3
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; //4
    fn approve(self: Box<Self>) -> Box<dyn State>; //7
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        "" //1
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {}) //5
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self //1
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self //6
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {}) //2
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
}
