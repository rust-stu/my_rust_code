use anyhow::Result;
use reqwest::{header, Client};

/// 创建预配置的HTTP客户端
pub fn create_http_client() -> Result<Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Magic CLI".parse()?);

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
} 