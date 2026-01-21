use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Webhook 分类
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookCategory {
    pub id: i64,
    pub title: String,
}

/// Webhook 订阅源
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookFeed {
    pub id: i64,
    pub user_id: i64,
    pub category_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<WebhookCategory>,
    pub feed_url: String,
    pub site_url: String,
    pub title: String,
    pub checked_at: DateTime<Utc>,
}

/// 附件
#[derive(Debug, Deserialize, Serialize)]
pub struct Enclosure {
    pub id: i64,
    pub user_id: i64,
    pub entry_id: i64,
    pub url: String,
    pub size: i64,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_progression: Option<String>,
}

/// 附件列表
pub type EnclosureList = Vec<Enclosure>;

/// Webhook 条目
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookEntry {
    pub id: i64,
    pub user_id: i64,
    pub feed_id: i64,
    pub status: String,
    pub hash: String,
    pub title: String,
    pub url: String,
    pub comments_url: String,
    #[serde(rename = "published_at")]
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub changed_at: DateTime<Utc>,
    pub content: String,
    pub author: String,
    pub share_code: String,
    pub starred: bool,
    pub reading_time: i32,
    pub enclosures: EnclosureList,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<WebhookFeed>,
}

/// 新条目事件
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookNewEntriesEvent {
    pub event_type: String,
    pub feed: WebhookFeed,
    pub entries: Vec<WebhookEntry>,
}

/// 保存条目事件（预留，未来可能使用）
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct WebhookSaveEntryEvent {
    pub event_type: String,
    pub entry: WebhookEntry,
}

// ====== 向后兼容的类型别名 ======

/// @deprecated 使用 WebhookFeed 替代
#[allow(dead_code)]
pub type MinifluxFeed = WebhookFeed;

/// @deprecated 使用 WebhookEntry 替代
#[allow(dead_code)]
pub type MinifluxEntry = WebhookEntry;

/// @deprecated 使用 WebhookNewEntriesEvent 替代
#[allow(dead_code)]
pub type MinifluxWebhook = WebhookNewEntriesEvent;
