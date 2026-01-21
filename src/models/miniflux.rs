use serde::Deserialize;

// 定义 Miniflux Webhook 发送的 JSON 结构
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct MinifluxEntry {
    pub id: u64,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub comments_url: String,
    #[serde(default)]
    pub published_at: String,  // RFC3339 格式时间，如 "2023-08-17T19:29:22Z"
    pub author: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct MinifluxFeed {
    pub id: u64,
    pub title: String,
    pub feed_url: String,
    pub site_url: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct MinifluxWebhook {
    pub event_type: String,
    pub feed: MinifluxFeed,
    pub entries: Vec<MinifluxEntry>,
}
