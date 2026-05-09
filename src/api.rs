use serde::Deserialize;

use ::readable_readability::Readability;

#[derive(Deserialize)]
pub struct Story {
    pub title: String,
    pub url: String,
    pub score: i32,
    pub comment_count: i32,
    pub tags: Vec<String>,
    pub submitter_user: String,
    pub comments_url: String,
    pub short_id: String,
}

#[derive(Deserialize)]
struct StoryDetail {
    comments: Vec<Comment>,
}

#[derive(Deserialize)]
pub struct Comment {
    pub comment: String,
    pub commenting_user: String,
    pub score: i32,
    pub depth: u32,
    pub created_at: String,
}

// lobste.rs home page fetching
pub async fn fetch() -> anyhow::Result<Vec<Story>> {
    let stories = reqwest::get("https://lobste.rs/hottest.json")
        .await?
        .json::<Vec<Story>>()
        .await?;
    Ok(stories)
}

// lobste.rs comment fetching
pub async fn fetch_comments(short_id: &str) -> anyhow::Result<Vec<Comment>> {
    let url = format!("https://lobste.rs/s/{}.json", short_id);
    let body = reqwest::get(&url).await?.json::<StoryDetail>().await?;
    Ok(body.comments)
}

// story fetching from html
pub fn extract_article(html: &str, url: &str) -> Option<String> {
    let url = url::Url::parse(url).ok()?;

    let (node, _metadata) = Readability::new().base_url(url).parse(html);
    let text = node.text_contents();
    Some(text.trim().to_string())
}
