use std::str::FromStr;
use std::io::{self, Read};

use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::{header, Method, Url};
use serde_json::{json, Value};


#[derive(Parser)]
#[clap(author="lan", version="0.1.0", about="A CLI for Custom tools", long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Httpie(HttpieArgs),
    Json(JsonArgs),
}

#[derive(Parser, Debug)]
struct HttpieArgs {
    #[clap(subcommand)]
    subcmd: HttpieSubCommand,
}

#[derive(Parser, Debug)]
enum HttpieSubCommand {
    Get(HttpieGetArgs),
    Post(HttpiePostArgs),
}

#[derive(Parser, Debug)]
struct HttpieGetArgs {
    #[clap(value_parser = parse_url)]
    #[clap(short, long)]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Parser, Debug)]
struct HttpiePostArgs {
    #[clap(value_parser = parse_url)]
    #[clap(short, long)]
    url: String,
    
    #[clap(short, long)]
    #[clap(value_parser = parse_kv_pair)]
    #[clap(help = "Key-value pairs for form data (e.g., key=value)")]
    form: Vec<KvPair>,
    
    #[clap(short = 'j', long = "json")]
    #[clap(help = "JSON data for request body")]
    json: Option<String>,
    
    #[clap(short = 'd', long = "data")]
    #[clap(help = "Raw data for request body")]
    data: Option<String>,
    
    #[clap(long = "content-type")]
    #[clap(default_value = "application/json")]
    #[clap(help = "Content-Type header")]
    content_type: String,
}

#[derive(Parser, Debug)]
struct JsonArgs {
    #[clap(help = "JSON 查询表达式 (类似 jq 语法)")]
    query: Option<String>,
    
    #[clap(short = 'c', long)]
    #[clap(help = "紧凑输出格式")]
    compact: bool,
    
    #[clap(short = 'o', long)]
    #[clap(help = "彩色输出")]
    color: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse key=value: {}", s));
        Ok(Self {
            k: split.next().ok_or_else(err)?.to_string(),
            v: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn query_json_path(value: &Value, path: &str) -> Result<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(obj) => {
                current = obj.get(part)
                    .ok_or_else(|| anyhow!("Key '{}' not found", part))?;
            }
            Value::Array(arr) => {
                let index: usize = part.parse()
                    .map_err(|_| anyhow!("Invalid array index: {}", part))?;
                current = arr.get(index)
                    .ok_or_else(|| anyhow!("Array index {} out of bounds", index))?;
            }
            _ => return Err(anyhow!("Cannot access '{}' on non-object/non-array", part)),
        }
    }
    
    Ok(current.clone())
}

#[derive(Debug)]
enum BodyData {
    Json(String),
    Form(Vec<KvPair>),
    Raw(String),
}

impl BodyData {
    fn content_type(&self) -> &str {
        match self {
            BodyData::Json(_) => "application/json",
            BodyData::Form(_) => "application/x-www-form-urlencoded",
            BodyData::Raw(_) => "text/plain",
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            BodyData::Json(data) => data.clone(),
            BodyData::Form(pairs) => {
                pairs.iter()
                    .map(|pair| format!("{}={}", pair.k, pair.v))
                    .collect::<Vec<_>>()
                    .join("&")
            }
            BodyData::Raw(data) => data.clone(),
        }
    }
}


async fn send_request(client: &reqwest::Client, method: &str, url: &str, body: Option<BodyData>) -> Result<()> {
    let mut req = client.request(Method::from_str(method)?, url);
    if let Some(body) = body {
        req = req.body(body.to_string());
    }
    let resp = req.send().await?;
    println!("Status: {}", resp.status());
    println!("Headers: {:?}", resp.headers());
    let body = resp.text().await?;
    println!("Body: {}", body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match args.subcommand {
        Command::Httpie(httpie_args) => {
            match httpie_args.subcmd {
                HttpieSubCommand::Get(httpie_get_args) => {
                    println!("GET request to {}", httpie_get_args.url);
                    send_request(&client, "GET", &httpie_get_args.url, None).await?;
                }
                HttpieSubCommand::Post(httpie_post_args) => {
                    println!("POST request to {}", httpie_post_args.url);
                    
                    let body_data = if let Some(json) = &httpie_post_args.json {
                        Some(BodyData::Json(json.clone()))
                    } else if !httpie_post_args.form.is_empty() {
                        Some(BodyData::Form(httpie_post_args.form.clone()))
                    } else if let Some(data) = &httpie_post_args.data {
                        Some(BodyData::Raw(data.clone()))
                    } else {
                        None
                    };
                    
                    if let Some(body) = &body_data {
                        println!("Body: {:?}", body);
                        println!("Content-Type: {}", body.content_type());
                        println!("Body string: {}", body.to_string());
                    } else {
                        println!("No body data provided");
                    }
                    send_request(&client, "POST", &httpie_post_args.url, body_data).await?;
                }
            }
        }
        Command::Json(json_args) => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            let value: Value = serde_json::from_str(&input)?;

            if let Some(query) = &json_args.query {
                // 简单的路径查询实现
                let result = query_json_path(&value, query)?;
                if json_args.compact {
                    println!("{}", serde_json::to_string(&result)?);
                } else {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            } else {
                if json_args.compact {
                    println!("{}", serde_json::to_string(&value)?);
                } else {
                    println!("{}", serde_json::to_string_pretty(&value)?);
                }
            }
        }
    };

    Ok(result)
}
