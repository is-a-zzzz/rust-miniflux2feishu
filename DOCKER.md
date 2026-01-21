# Docker 部署指南

## 快速开始

### 1. 配置环境变量

```bash
# 复制环境变量模板
cp .env.example .env

# 编辑 .env 文件，填入你的飞书 Webhook URL
vim .env
```

### 2. 构建并启动服务

```bash
# 构建镜像并启动
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

### 3. 测试服务

```bash
curl -X POST http://127.0.0.1:8083/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "feed_title": "测试订阅源",
    "entries": [
      {
        "title": "测试文章",
        "url": "https://example.com/article",
        "comments_url": "https://miniflux.example.com/entry/1"
      }
    ]
  }'
```

## 环境变量说明

| 变量名 | 必填 | 默认值 | 说明 |
|--------|------|--------|------|
| `WEBHOOK_URL` | ✅ | - | 飞书机器人 Webhook URL |
| `RUST_LOG` | ❌ | `info` | 日志级别 (debug/info/warn/error) |
| `IP` | ❌ | `0.0.0.0` | 监听地址 |
| `PORT` | ❌ | `8083` | 监听端口 |

## 镜像大小优化

Dockerfile 使用多阶段构建策略：
- **构建阶段**: 使用 `rust:1.83-alpine` 编译
- **运行阶段**: 使用 `alpine:3.20`，仅包含二进制文件和 CA 证书

最终镜像大小约 **5-8 MB**

## 生产环境部署建议

### 1. 使用反向代理

推荐使用 Nginx/Caddy 作为反向代理：

```nginx
# nginx.conf
location /webhook {
    proxy_pass http://127.0.0.1:8083;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}
```

### 2. 健康检查

在 docker-compose.yml 中添加：

```yaml
healthcheck:
  test: ["CMD", "wget", "--spider", "http://localhost:8083/webhook"]
  interval: 30s
  timeout: 10s
  retries: 3
```

### 3. 日志管理

限制日志大小：

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```
