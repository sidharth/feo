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

## TODOs

Ingestion:  
[✅] Read all Markdown files from ~/posts/  
[✅] Parse all content into Post struct types.  

Generation:  
[✅] Define post template.  
[✅] Generate posts from post array.  
[ ] Define index template.  
[ ] Generate index from post array.  

Styling:
[] Make default posts page look good.
[] Make default index page look good.