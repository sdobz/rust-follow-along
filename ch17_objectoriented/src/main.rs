extern crate ch17_objectoriented;

use ch17_objectoriented::blog_oo::Post as PostOO;
use ch17_objectoriented::blog_types::Post as PostTypes;
use ch17_objectoriented::AveragedCollection;
use ch17_objectoriented::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        let opts = self.options.join(", ");

        println!("Drawing select box for {}", opts);
    }
}

fn main() {
    let mut avg_collection = AveragedCollection::new();

    avg_collection.add(5);
    avg_collection.add(7);
    avg_collection.add(10);
    if let Some(value) = avg_collection.remove() {
        println!("Removed: {}", value);
    }

    println!("Average: {}", avg_collection.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("opts"),
                    String::from("for"),
                    String::from("selectbox"),
                ],
            }),
            Box::new(Button {
                width: 123,
                height: 456,
                label: String::from("hello there"),
            }),
        ],
    };

    screen.run();

    let mut post = PostOO::new();

    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());
    println!("PostOO content: {}", post.content());

    let mut post = PostTypes::new();

    post.add_text("I ate like 6 salads for lunch today");
    // assert_eq!("", post.content()); - err, DraftPost doesn't have .content()

    let post = post.request_review();
    // assert_eq!("", post.content()); - err, PendingReviewPost doesn't have .content()

    let post = post.approve();
    assert_eq!("I ate like 6 salads for lunch today", post.content());
    println!("PostTypes content: {}", post.content());
}
