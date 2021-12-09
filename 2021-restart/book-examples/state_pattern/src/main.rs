use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate something");
    assert_eq!("", post.content());
    println!("Done")
}
