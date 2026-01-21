#!/bin/bash
# 构建带日期标签的 Docker 镜像

DATE=$(date +%Y%m%d)
IMAGE_NAME="rust-miniflux2feishu"

echo "构建镜像: ${IMAGE_NAME}:${DATE}"
docker build -t ${IMAGE_NAME}:${DATE} -t ${IMAGE_NAME}:latest .

echo "镜像列表:"
docker images | grep ${IMAGE_NAME}

echo ""
echo "使用方式:"
echo "  docker run -d -p 8083:8083 --name miniflux2feishu ${IMAGE_NAME}:${DATE}"
echo "  或"
echo "  docker compose up -d  (使用 latest 标签)"
