use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct PostMeta {
    pub slug: String,
    pub title: String,
    pub publish_date: String,
    pub published: bool
}   

pub struct Post {
    pub meta: PostMeta,
    pub html: String
}