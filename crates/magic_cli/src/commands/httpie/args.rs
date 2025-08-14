use clap::Parser;
use crate::shared::types::KvPair;
use super::validators::{parse_http_url, parse_form_kv_pair};

#[derive(Parser, Debug)]
pub struct HttpieArgs {
    #[clap(subcommand)]
    pub subcmd: HttpieSubCommand,
}

#[derive(Parser, Debug)]
pub enum HttpieSubCommand {
    /// 发送GET请求
    Get(HttpieGetArgs),
    /// 发送POST请求
    Post(HttpiePostArgs),
}

#[derive(Parser, Debug)]
pub struct HttpieGetArgs {
    #[clap(value_parser = parse_http_url)]
    #[clap(short, long)]
    #[clap(help = "请求的URL地址")]
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct HttpiePostArgs {
    #[clap(value_parser = parse_http_url)]
    #[clap(short, long)]
    #[clap(help = "请求的URL地址")]
    pub url: String,
    
    #[clap(short, long)]
    #[clap(value_parser = parse_form_kv_pair)]
    #[clap(help = "表单数据的键值对 (例如: key=value)")]
    pub form: Vec<KvPair>,
    
    #[clap(short = 'j', long = "json")]
    #[clap(help = "请求体的JSON数据")]
    pub json: Option<String>,
    
    #[clap(short = 'd', long = "data")]
    #[clap(help = "请求体的原始数据")]
    pub data: Option<String>,
    
    #[clap(long = "content-type")]
    #[clap(default_value = "application/json")]
    #[clap(help = "Content-Type头")]
    pub content_type: String,
} 