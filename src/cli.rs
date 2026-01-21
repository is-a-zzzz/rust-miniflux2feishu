use clap::Parser;

/// Miniflux Webhook 转发到飞书机器人的服务
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 监听的 IP 地址
    #[arg(short = 'i', long, default_value = "0.0.0.0", env = "IP")]
    pub ip: String,

    /// 监听的端口
    #[arg(short = 'p', long, default_value_t = 8083, env = "PORT")]
    pub port: u16,

    /// 飞书机器人的 Webhook URL
    #[arg(short = 'w', long, env = "WEBHOOK_URL")]
    pub webhook_url: String,

    /// Miniflux 服务器地址
    #[arg(short = 'm', long, env = "MINIFLUX_URL", default_value = "")]
    pub miniflux_url: String,
}
