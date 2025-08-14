use clap::Parser;

#[derive(Parser, Debug)]
pub struct JsonArgs {
    #[clap(help = "JSON查询表达式 (类似jq语法)")]
    pub query: Option<String>,
    
    #[clap(short = 'c', long)]
    #[clap(help = "紧凑输出格式")]
    pub compact: bool,
    
    #[clap(short = 'o', long)]
    #[clap(help = "彩色输出")]
    pub color: bool,
} 