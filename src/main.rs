mod models;

fn main() {
    let post = models::Post {
        slug: String::from("hello-world"),
        title: String::from("Hello, world!"),
        content: String::from("Hello, world!"),
        publish_date: String::from("2019-01-01"),
        published: true
    };

    println!("Post: {}", post.slug);
}

