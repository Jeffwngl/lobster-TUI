use serde::Deserialize;

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

pub async fn fetch() -> anyhow::Result<Vec<Story>> {
    let stories = reqwest::get("https://lobste.rs/hottest.json")
        .await?
        .json::<Vec<Story>>()
        .await?;
    Ok(stories)
}
