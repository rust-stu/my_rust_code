use anyhow::Result;
use clap::Parser;
use magic_cli::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    args.command.execute().await
}
