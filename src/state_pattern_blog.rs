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

        // TODO: make this return a result?
        pub fn add_text(&mut self, text: &str) {
            // Need to use as_ref() here to avoid "cannot move out of mutable reference error".

            if let Some(state) = self.state.as_ref() {
                state.add_text(&mut self.content, text);
            }
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


    struct Draft {}
    struct PendingReview {}
    struct PendingFinalApproval {}
    struct Published {}


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
        fn add_text(&self, content: &mut String, text: &str) {}
    }

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

        fn add_text(&self, content: &mut String, text: &str) {
            content.push_str(text);
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
