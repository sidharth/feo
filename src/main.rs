extern crate yaml_rust;

mod models;

use handlebars::Handlebars;
use std::collections::BTreeMap;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let posts = parse_posts();

    for post in &posts {
        let generated_html = get_post_page(&post);
        // Create directory if it doesn't exist.
        std::fs::create_dir_all("gen").unwrap();
        // Save the generated HTML to a file.
        std::fs::write(format!("gen/{}.html", post.meta.slug), &generated_html).unwrap();
    }
    println!("Generated {} post(s).", posts.len());
    
}

fn get_post_page(post: &models::Post) -> String {
    // Get the post template from the file.
    let mut handlebars = Handlebars::new();
    let post_template = std::fs::read_to_string("templates/post.hbs").unwrap();
    // Register the template, and check that it's okay.
    assert!(handlebars.register_template_string("post_template", post_template).is_ok());

    // Put the post data into a 1:1 template map.
    let mut data = BTreeMap::new();
    data.insert("title", &post.meta.title);
    data.insert("publish_date", &post.meta.publish_date);
    data.insert("content", &post.html);

    // Render the data onto the template.
    return handlebars.render("post_template", &data).unwrap();
}

// Read all markdown files in the posts directory.
fn parse_posts() -> Vec<models::Post> {
    let mut posts = Vec::new();

    for entry in std::fs::read_dir("posts").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            // (0) Get the raw content from the file.
            let contents = std::fs::read_to_string(path).unwrap();

            // (1) Extract the frontmatter metadata.
            // Get the YAML metadata at the beginning of the post.

            println!("contents: {}", contents);

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

