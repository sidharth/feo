extern crate yaml_rust;

mod models;


fn main() {
    let posts = get_posts();
    // print number of posts.
    println!("Parsed {} post(s).", posts.len());
}

// Read all markdown files in the posts directory.
fn get_posts() -> Vec<models::Post> {
    let mut posts = Vec::new();

    for entry in std::fs::read_dir("posts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            // (0) Get the raw content from the file.
            let contents = std::fs::read_to_string(path).unwrap();

            // (1) Extract the frontmatter metadata.
            // Get the YAML metadata at the beginning of the post.
            let doc = &yaml_rust::YamlLoader::load_from_str(&contents).unwrap()[0];
            let frontmatter = markdown::Options {
                parse: markdown::ParseOptions {
                    constructs: markdown::Constructs {
                        frontmatter: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            };
            // Parse out the post metadata.
            let post = models::PostMeta {
                slug: doc["slug"].as_str().unwrap().to_string(),
                title: doc["title"].as_str().unwrap().to_string(),
                publish_date: doc["publish_date"].as_str().unwrap().to_string(),
                published: doc["published"].as_bool().unwrap()
            };

            // (2) Extract HTML from the markdown content.
            let html = markdown::to_html_with_options(&contents, &frontmatter);
            
            // (3) Combine it into a Post object.
            let post = models::Post {
                meta: post,
                html: html.unwrap()
            };

            // (4) Add it to the list of posts.
            posts.push(post)
        }
    }

    posts
}

