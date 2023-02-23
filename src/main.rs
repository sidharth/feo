extern crate yaml_rust;

mod models;

use handlebars::Handlebars;
use serde_json::json;
use std::collections::BTreeMap;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let posts = parse_posts();

    // Generate HTML for each post.
    for post in &posts {
        let generated_post_html = get_post_page(&post);
        // Create directory if it doesn't exist.
        std::fs::create_dir_all("gen").unwrap();
        // Save the generated HTML to a file.
        std::fs::write(format!("gen/{}.html", post.meta.slug), &generated_post_html).unwrap();
    }

    // Generate the index page.
    let generated_index_html = get_index_page(&posts);
    // Save the generated HTML to a file.
    std::fs::write("gen/index.html", &generated_index_html).unwrap();

    println!("Generated {} post(s).", posts.len());


    // Copy the CSS file to the gen directory.
    std::fs::create_dir_all("gen/styles").unwrap();
    std::fs::copy("styles/common.css", "gen/styles/common.css").unwrap();

    // Copy all of the files in assets/ into the gen/assets directory.
    std::fs::create_dir_all("gen/assets").unwrap();
    let assets_paths = get_asset_paths();
    for asset_path in assets_paths {
        // get last part of path.
        let file_name = asset_path.split("/").last().unwrap();

        std::fs::copy(&asset_path, format!("gen/assets/{}", file_name)).unwrap();
    }
}

fn get_index_page(posts: &Vec<models::Post>) -> String {
    // Get the index template from the file.
    let mut handlebars = Handlebars::new();
    let index_template = std::fs::read_to_string("templates/index.hbs").unwrap();
    // Register the template, and check that it's okay.
    assert!(handlebars.register_template_string("index_template", index_template).is_ok());

    // Put the post data into a render-able JSON.
    let data = json!({
        "post_meta": posts.iter().map(|p| {&p.meta}).collect::<Vec<&models::PostMeta>>()
    });

    // Render the data onto the template.
    return handlebars.render("index_template", &data).unwrap();
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

// Get all the site assets (eg. images).
fn get_asset_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            paths.push(path.to_str().unwrap().to_string());
        }
    }

    paths
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
            let yaml_contents = contents.split("---").nth(1).unwrap();

            // (1) Extract the frontmatter metadata.
            // Get the YAML metadata at the beginning of the post.
            let doc = &yaml_rust::YamlLoader::load_from_str(&yaml_contents).unwrap()[0];
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

    // Sort by publish date.
    posts.sort_by(|a, b| b.meta.publish_date.cmp(&a.meta.publish_date));

    posts
}

