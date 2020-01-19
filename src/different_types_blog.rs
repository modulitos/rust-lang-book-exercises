// This is using each transition as a different type
pub mod blog {
    pub struct Post {
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

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReview {
            PendingReview {
                content: self.content,
            }
        }
    }

    pub struct PendingReview {
        content: String,
    }

    impl PendingReview {
        pub fn approve(self) -> PendingSecondReview {
            PendingSecondReview {
                content: self.content,
            }
        }
    }

    pub struct PendingSecondReview {
        content: String,
    }

    impl PendingSecondReview {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::blog::*;

    #[test]
    fn approve_posts() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve(); // first approval
        let post = post.approve(); // final approval

        assert_eq!("I ate a salad for lunch today", post.content());

        let post = post.reject();
    }
}

