pub mod httpie;
pub mod json;

use async_trait::async_trait;
use anyhow::Result;
use clap::Parser;

/// 所有命令的统一接口
#[async_trait]
pub trait Command {
    type Args;
    async fn execute(&self, args: Self::Args) -> Result<()>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

/// 所有可用的命令
#[derive(Parser, Debug)]
pub enum Commands {
    /// HTTP客户端工具
    Httpie(httpie::HttpieArgs),
    /// JSON处理工具
    Json(json::JsonArgs),
}

impl Commands {
    pub async fn execute(self) -> Result<()> {
        match self {
            Commands::Httpie(args) => {
                let cmd = httpie::HttpieCommand;
                cmd.execute(args).await
            }
            Commands::Json(args) => {
                let cmd = json::JsonCommand;
                cmd.execute(args).await
            }
        }
    }
} 