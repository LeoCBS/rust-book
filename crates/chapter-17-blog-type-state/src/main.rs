use chapter_17_blog_type_state::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("i ate a salad today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
