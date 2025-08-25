use blog1701::Post;

fn main() {
    let mut post = Post::new(); //1

    post.add_text("I ate a helemian for lunch today"); //2
    assert_eq!("", post.content()); //3

    post.request_review(); //4
    assert_eq!("", post.content()); //5

    post.approve(); //6
    assert_eq!("I ate a helemian for lunch today", post.content()); //7
}
