use traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet{
        username: "horse_books".to_string(),
        content: "of course, as ... whatever".to_string(),
        reply: false,
        retweet:  false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
