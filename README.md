# Feo
A no-frills static site generator written in Rust.

### Writing

Write all your posts in Markdown (i.e. with an .md extension) in `/posts`.  Make sure to start each post with the YAML metadata to help the generator. 

```
---
slug: getting-rusty
title: Getting Rusty
published: true
publish_date: 2023-02-15
---

Long years ago, we made a tryst with destiny. Now the time comes, we must redeem our pledge...
```

### Building

Run this from the root directory of the project.

```
cargo run
```

This generates static HTML pages for all posts + the index + CSS styles inside the `gen/` folder.

### Styling.

Add styles in `styles/common.css`. During the build, it copies the stylesheet to `gen/styles/`.  

### Assets

Add assets (images, raw files etc) in `assets/`. During the build, it copies assets to `gen/assets/`.


## TODOs

Ingestion:  
[✅] Read all Markdown files from ~/posts/  
[✅] Parse all content into Post struct types.  

Generation:  
[✅] Define base post template.  
[✅] Generate posts from post array.  
[✅] Define base index template.  
[✅] Generate index from post array.  

Styling:  
[] Make default posts page look good.  
[] Make default index page look good.  

Bugs:  
[] Can't include `:` in posts because it gets caught by YAML parser...