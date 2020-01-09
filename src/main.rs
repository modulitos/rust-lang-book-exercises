mod state_pattern_blog;

pub use crate::state_pattern_blog::blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("Not this!"); // content can only be added to Drafts

    post.approve(); // first approval
    assert_eq!("", post.content());

    post.approve(); // final approval
    assert_eq!("I ate a salad for lunch today", post.content());

    post.reject();
    assert_eq!("", post.content());
}
