struct Draft {}
struct PendingReview {}
struct Published {}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    // Box<Self> is used to eliminate the 'self', '&self' or '&mut self'
    // Ownership of Box invalidating the state value of the Post
    // This will invalidate the old state of Post, so it can transform to
    // a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    // Default implementation for content is to return an empty string
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // Get the content from the state
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // To transform, we need to take ownership of old state.
        // Option does this; we call .take() from the Option
        // This returns a `Some` and leave a `None` in place.
        // Makes it easier than borrowing.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        // If a state exists
        if let Some(s) = self.state.take() {
            // Call the reject function
            self.state = Some(s.reject())
        }
    }
}

impl State for Draft {
    // Needs to return a new boxed instance of PendingReview which also
    // implements the State trait.
    // PendingReview just returns itself, since it doesn't transform.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Calling approve on a draft will have no effect, only return itself.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // A pending reveiew will return the next published post.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
