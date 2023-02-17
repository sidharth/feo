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
    read_posts();
}

// Read all markdown files in the posts directory.
fn read_posts() -> Vec<models::Post> {
    let mut posts = Vec::new();

    for entry in std::fs::read_dir("posts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            print!("Reading file: {}\n", path.display());
            let contents = std::fs::read_to_string(path).unwrap();
            print!("Contents: {}", contents)
        }
    }

    posts
}

