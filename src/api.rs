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
}

// lobste.rs page fetching
pub async fn fetch() -> anyhow::Result<Vec<Story>> {
    let stories = reqwest::get("https://lobste.rs/hottest.json")
        .await?
        .json::<Vec<Story>>()
        .await?;
    Ok(stories)
}

// story fetching from html
pub fn extract_article(html: &str, url: &str) -> Option<String> {
    let url = url::Url::parse(url).ok()?;

    let (node, _metadata) = Readability::new().base_url(url).parse(html);
    let text = node.text_contents();
    Some(text.trim().to_string())
}
