use std::{fs, path::Path};

use chrono::{DateTime, TimeZone, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

pub fn retrieve_posts() -> Option<Json<Vec<Post>>> {
    let mut posts: Vec<_> = fs::read_dir("posts")
        .ok()?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.is_file() && path.extension().unwrap_or_default() == "md")
        .filter_map(|path| Post::read_post(&path))
        .collect();

    // Sort all the posts. We want the most recent ones first.
    posts.sort_by_key(|p| p.date.timestamp());
    posts.reverse();

    Some(Json(posts))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    date: DateTime<Utc>,
    body: String,
}

impl Post {
    pub fn read_post(path: &Path) -> Option<Self> {
        let contents = fs::read_to_string(path).ok()?;
        Self::try_parse(&contents)
    }

    fn try_parse(md: &str) -> Option<Self> {
        Some(Post {
            title: extract_post_title(md)?,
            date: Utc.timestamp(extract_post_timestamp(md)?, 0),
            body: render_post(md)?,
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

fn render_post<P: AsRef<Path>>(path: P) -> Option<String> {
    let post = fs::read_to_string(path).ok()?;
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    //options.render.unsafe_ = true;

    Some(markdown_to_html(&post, &options))
}
