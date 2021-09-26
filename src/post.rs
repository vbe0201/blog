use std::{fs, path::Path};

use chrono::{DateTime, TimeZone, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

pub fn retrieve_all() -> Option<JsonValue> {
    let mut posts: Vec<JsonValue> = fs::read_dir("posts")
        .ok()?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "md")
        .filter_map(|path| Post::read_post(&path).map(Into::into))
        .collect();

    // Sort all the posts. We want the most recent ones first.
    posts.sort_by_key(|p| p["timestamp"].as_i64());
    posts.reverse();

    Some(json!({ "posts": posts }))
}

pub fn get(name: String) -> Option<JsonValue> {
    let path = Path::new("posts").join(name + ".md");
    match path.is_file() {
        true => Post::read_post(&path).map(|p| json!({ "post": Into::<JsonValue>::into(p) })),
        false => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    name: String,
    title: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    date: DateTime<Utc>,
    body: String,
}

impl Post {
    pub fn read_post(path: &Path) -> Option<Self> {
        let contents = fs::read_to_string(path).ok()?;
        Self::try_parse(path, &contents)
    }

    fn try_parse(path: &Path, md: &str) -> Option<Self> {
        Some(Post {
            name: path.file_stem()?.to_str()?.to_owned(),
            title: extract_post_title(md)?,
            date: Utc.timestamp(extract_post_timestamp(md)?, 0),
            body: render_post(md)?,
        })
    }
}

impl From<Post> for JsonValue {
    fn from(post: Post) -> Self {
        json!({
            "name": post.name,
            "title": post.title,
            "date": post.date.format("%d %B %Y").to_string(),
            "timestamp": post.date.timestamp(),
            "body": post.body,
        })
    }
}

fn extract_post_title(md: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^\s*#(.+)").unwrap();
    }

    Some(RE.captures(md)?.get(1)?.as_str().into())
}

fn extract_post_timestamp(md: &str) -> Option<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<!--\s*timestamp:\s*(\d+)\s*-->").unwrap();
    }

    RE.captures(md)?.get(1)?.as_str().parse::<i64>().ok()
}

fn render_post(post: &str) -> Option<String> {
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    //options.render.unsafe_ = true;

    Some(markdown_to_html(post, &options))
}
