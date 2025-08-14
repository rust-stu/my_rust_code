use clap::Parser;
use crate::commands::Commands;

#[derive(Parser)]
#[clap(author="lan", version="0.1.0", about="一个自定义工具的CLI", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
} 