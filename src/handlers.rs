use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

use crate::state::AppState;
use crate::models::{
    miniflux::MinifluxWebhook,
    lark::build_lark_payload,
};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000; // 1秒延迟
const MESSAGE_INTERVAL_MS: u64 = 2500; // 消息间隔2.5秒，避免触发飞书429限流

pub async fn handle_miniflux_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MinifluxWebhook>,
) -> StatusCode {
    if payload.entries.is_empty() {
        return StatusCode::OK; // 没有新文章，正常返回
    }

    info!(
        "接收到 Miniflux 更新：{}，共 {} 篇文章",
        payload.feed.title,
        payload.entries.len()
    );

    // 处理所有文章，每篇文章单独发送
    let mut success_count = 0;
    let mut failed_count = 0;

    for (index, entry) in payload.entries.iter().enumerate() {
        info!(
            "处理第 {}/{} 篇文章：{}",
            index + 1,
            payload.entries.len(),
            entry.title
        );

        // 构造飞书消息体
        let lark_payload = build_lark_payload(entry, &state.miniflux_url);

        // 尝试发送，支持429重试
        let mut retries = 0;
        loop {
            match send_to_lark(&state, &lark_payload).await {
                Ok(true) => {
                    // 发送成功
                    info!("成功发送第 {} 篇文章到飞书", index + 1);
                    success_count += 1;

                    // 如果不是最后一篇文章，添加延迟避免429限流
                    if index < payload.entries.len() - 1 {
                        info!("等待 {}ms 后发送下一条消息...", MESSAGE_INTERVAL_MS);
                        sleep(Duration::from_millis(MESSAGE_INTERVAL_MS)).await;
                    }

                    break;
                }
                Ok(false) => {
                    // 429错误，需要重试（使用指数退避）
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        error!("第 {} 篇文章发送失败：超过最大重试次数", index + 1);
                        failed_count += 1;
                        break;
                    }
                    // 指数退避：第1次1秒，第2次2秒，第3次4秒
                    let backoff_ms = RETRY_DELAY_MS * 2_u64.pow(retries - 1);
                    warn!(
                        "遇到429限流，第 {} 次重试（{}ms 后）...",
                        retries, backoff_ms
                    );
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
                Err(e) => {
                    // 其他错误，不重试
                    error!("第 {} 篇文章发送失败：{}", index + 1, e);
                    failed_count += 1;
                    break;
                }
            }
        }
    }

    info!(
        "发送完成：成功 {} 篇，失败 {} 篇",
        success_count, failed_count
    );

    if failed_count > 0 {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    }
}

async fn send_to_lark(
    state: &Arc<AppState>,
    lark_payload: &crate::models::lark::LarkMessage,
) -> Result<bool, String> {
    let response = state
        .http_client
        .post(&state.lark_webhook_url)
        .json(lark_payload)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let status = response.status();

    if status.is_success() {
        Ok(true)
    } else if status.as_u16() == 429 {
        // 429 Too Many Requests，需要重试
        Ok(false)
    } else {
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "无法读取响应体".to_string());
        Err(format!("状态码 {}，响应：{}", status, text))
    }
}
