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
    Text { text: String },
    A { text: String, href: String },
    At { user_id: String },
}

// --- 辅助函数 ---

// 格式化 RFC3339 时间字符串
fn format_published_time(published_at: &str) -> String {
    if published_at.is_empty() {
        return String::new();
    }

    // 尝试解析 RFC3339 格式时间
    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(published_at) {
        // 转换为北京时间 (UTC+8)
        let beijing_time = datetime.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
        // 格式化为：2023-08-17 19:29
        beijing_time.format("%Y-%m-%d %H:%M").to_string()
    } else {
        published_at.to_string()
    }
}

// --- 构造飞书消息函数 ---

pub fn build_lark_payload(entry: &MinifluxEntry, miniflux_url: &str) -> LarkMessage {
    // 构建消息内容
    let mut content = vec![];

    // 如果有发布时间，显示时间
    if !entry.published_at.is_empty() {
        let time_str = format_published_time(&entry.published_at);
        if !time_str.is_empty() {
            content.push(vec![
                LarkElement::Text {
                    text: format!("📅 {}", time_str),
                },
            ]);
        }
    }

    // Miniflux访问地址（用于标记已读）
    if !miniflux_url.is_empty() {
        let miniflux_entry_url = format!("{}/rss/entry/{}", miniflux_url.trim_end_matches('/'), entry.id);
        tracing::info!("构造 Miniflux URL: {} (entry.id={})", miniflux_entry_url, entry.id);
        content.push(vec![
            LarkElement::A {
                text: "📱 Miniflux 查看".to_string(),
                href: miniflux_entry_url,
            },
        ]);
    }

    // 原始文章地址
    content.push(vec![
        LarkElement::A {
            text: "🔗 原文链接".to_string(),
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
