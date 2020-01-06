use blog::Post;

// This is using the "state pattern":
pub mod blog {

    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>,
    }

    impl Post {
        pub fn new() -> Self {
            Post {
                content: String::new(),
                state: Some(Box::new(Draft {})),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            // satisfies the borrow checker, in place of:
            // self.state = self.state.request_review();
            if let Some(state) = self.state.take() {
                self.state = Some(state.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.approve());
            }
        }

        pub fn reject(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.reject());
            }
        }
    }

    trait State {
        // We might update these methods with default implementations that return self, however,
        // this would violate object safety, because the trait doesn't know what the concrete self
        // will be exactly. We want to be able to use State as a trait object, so we need its
        // methods to be object safe.

        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }

        fn reject(self: Box<Self>) -> Box<dyn State>;
    }

    struct Draft {}
    struct PendingReview {}
    struct PendingFinalApproval {}
    struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
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
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingFinalApproval {})
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl State for PendingFinalApproval {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
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
            Box::new(PendingReview {})
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve(); // first approval
    assert_eq!("", post.content());

    post.approve(); // final approval
    assert_eq!("I ate a salad for lunch today", post.content());

    post.reject();
    assert_eq!("", post.content());
}
