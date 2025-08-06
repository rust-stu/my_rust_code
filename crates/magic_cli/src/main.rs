use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::{header, Method, Url};


#[derive(Parser)]
#[clap(author="lan", version="0.1.0", about="A CLI for Custom tools", long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Httpie(HttpieArgs),
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
    };

    Ok(result)
}
