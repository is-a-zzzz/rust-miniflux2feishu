use serde::Serialize;
use crate::models::miniflux::MinifluxEntry;

// 飞书消息的顶层结构
#[derive(Debug, Serialize)]
pub struct LarkMessage {
    pub msg_type: &'static str,
    pub content: LarkContent,
}

#[derive(Debug, Serialize)]
pub struct LarkContent {
    pub post: LarkPost,
}

#[derive(Debug, Serialize)]
pub struct LarkPost {
    pub zh_cn: LarkLanguageContent,
}

#[derive(Debug, Serialize)]
pub struct LarkLanguageContent {
    pub title: String,
    pub content: Vec<Vec<LarkElement>>,
}

// 飞书支持的元素类型
#[derive(Debug, Serialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum LarkElement {
    #[allow(dead_code)]
    Text { text: String },
    A { text: String, href: String },
    #[allow(dead_code)]
    At { user_id: String },
}

// --- 辅助函数 ---

// 格式化 DateTime 为北京时间字符串（预留，暂未使用）
#[allow(dead_code)]
fn format_published_time(date: &chrono::DateTime<chrono::Utc>) -> String {
    // 转换为北京时间 (UTC+8)
    let beijing_time = date.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
    // 格式化为：2025-01-21 10:30:00
    beijing_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

// --- 构造飞书消息函数 ---

pub fn build_lark_payload(entry: &MinifluxEntry, miniflux_url: &str) -> LarkMessage {
    // 构建消息内容
    let mut content = vec![];

    // Miniflux文章地址
    if !miniflux_url.is_empty() {
        let miniflux_entry_url = format!("{}/rss/feed/{}/entry/{}", miniflux_url.trim_end_matches('/'), entry.feed_id, entry.id);
        tracing::info!("构造 Miniflux URL: {} (feed_id={}, entry_id={})", miniflux_entry_url, entry.feed_id, entry.id);
        content.push(vec![
            LarkElement::A {
                text: "Miniflux".to_string(),
                href: miniflux_entry_url,
            },
        ]);
    }

    // 原始文章地址
    content.push(vec![
        LarkElement::A {
            text: "原文".to_string(),
            href: entry.url.clone(),
        },
    ]);

    LarkMessage {
        msg_type: "post",
        content: LarkContent {
            post: LarkPost {
                zh_cn: LarkLanguageContent {
                    title: entry.title.clone(),
                    content,
                },
            },
        },
    }
}
