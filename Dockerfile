# ============================================
# 多架构构建阶段
# ============================================
FROM --platform=linux/amd64 messense/rust-musl-cross:x86_64-musl AS amd64
COPY . .

# 优化编译：LTO + strip
RUN CARGO_PROFILE_RELEASE_LTO=true \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
    CARGO_PROFILE_RELEASE_OPT_LEVEL=z \
    CARGO_PROFILE_RELEASE_STRIP=true \
    cargo install --path . --root /

# 进一步 strip 去除符号
RUN x86_64-linux-musl-strip /bin/miniflux-webhook || true

# ============================================
# 运行阶段：使用 scratch 裸镜像
# ============================================
FROM scratch
COPY --from=amd64 /bin/miniflux-webhook /miniflux-webhook

# 暴露端口
EXPOSE 8083

# 启动应用
ENTRYPOINT ["/miniflux-webhook"]
