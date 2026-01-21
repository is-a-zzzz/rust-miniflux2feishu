# rust-miniflux2feishu

将 Miniflux RSS 订阅更新实时推送到飞书机器人的转发服务。

## 功能特性

- 🚀 实时接收 Miniflux webhook 通知
- 📱 推送到飞书群机器人
- 🔗 自动生成 Miniflux 文章链接
- 🛡️ 429 限流自动重试（指数退避）
- 📦 轻量级 Docker 镜像 (~2MB)
- ⚡ Rust 实现，高性能稳定

## 快速开始

### 方式一：Docker 部署（推荐）

```bash
# 1. 复制配置模板
cp .env.example .env

# 2. 编辑配置，设置飞书 Webhook URL
vim .env
```

`.env` 文件内容：
```bash
# 必填：飞书机器人 Webhook URL
WEBHOOK_URL=https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-url

# 可选：Miniflux 服务器地址（用于生成文章链接）
MINIFLUX_URL=https://miniflux.example.com

# 可选：其他配置
RUST_LOG=info
IP=0.0.0.0
PORT=8083
```

```bash
# 3. 启动服务
docker compose up -d

# 4. 查看日志
docker compose logs -f
```

### 方式二：本地运行

```bash
# 编译
cargo build --release

# 启动
./target/release/miniflux-webhook \
  -w https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-url \
  -m https://miniflux.example.com
```

## 配置说明

| 参数 | 环境变量 | 必填 | 默认值 | 说明 |
|------|----------|------|--------|------|
| `-w, --webhook-url` | `WEBHOOK_URL` | ✅ | - | 飞书机器人 Webhook URL |
| `-m, --miniflux-url` | `MINIFLUX_URL` | ❌ | 空 | Miniflux 服务器地址 |
| `-i, --ip` | `IP` | ❌ | 0.0.0.0 | 监听地址 |
| `-p, --port` | `PORT` | ❌ | 8083 | 监听端口 |
| - | `RUST_LOG` | ❌ | info | 日志级别 |

### 获取飞书 Webhook URL

1. 打开飞书群
2. 群设置 → 群机器人 → 添加机器人 → 自定义机器人
3. 复制 Webhook URL

## Miniflux 配置

在 Miniflux 中设置 Webhook：

```
https://your-server.com:8083/webhook
```

## Webhook 格式

### 请求格式

Miniflux 会发送 POST 请求到 `/webhook` 端点：

```json
{
  "event_type": "new_entries",
  "feed": {
    "id": 22,
    "title": "示例订阅源"
  },
  "entries": [
    {
      "id": 1307,
      "feed_id": 22,
      "title": "文章标题",
      "url": "https://example.com/article",
      "published_at": "2025-01-21T10:00:00Z",
      "content": "<p>文章内容...</p>",
      "author": "作者"
    }
  ]
}
```

### 飞书推送格式

每篇文章会单独推送一张卡片：

```
【文章标题】
📅 2025-01-21 10:00
📱 Miniflux 查看
🔗 原文链接
```

其中 **📱 Miniflux 查看** 链接格式：
```
https://miniflux.example.com/rss/feed/{feed_id}/entry/{entry_id}
```

## 测试

使用提供的测试数据：

```bash
curl -X POST http://127.0.0.1:8083/webhook \
  -H "Content-Type: application/json" \
  -d @test_payload.json
```

## 限流处理

服务自动处理飞书 429 限流：

- 最多重试 3 次
- 指数退避：1秒 → 2秒 → 4秒
- 消息间延迟 2.5 秒

## Docker 镜像

- **基础镜像**: scratch
- **架构**: linux/amd64
- **大小**: 约 2-3 MB

## 生产环境建议

### 1. 使用反向代理

```nginx
location /webhook {
    proxy_pass http://127.0.0.1:8083;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}
```

### 2. 日志限制

在 `docker-compose.yml` 中添加：

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

### 3. 自动重启

```yaml
restart: unless-stopped
```

## 开发

```bash
# 运行
cargo run

# 测试
cargo test

# 构建
cargo build --release
```

## License

MIT
