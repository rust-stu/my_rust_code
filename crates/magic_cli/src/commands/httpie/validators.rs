use anyhow::Result;
use reqwest::Url;

/// 验证HTTP URL格式
pub fn parse_http_url(s: &str) -> Result<String> {
    let url: Url = s.parse()?;
    
    // 验证协议是否为http或https
    if !matches!(url.scheme(), "http" | "https") {
        return Err(anyhow::anyhow!("URL必须使用http或https协议"));
    }
    
    Ok(s.into())
}

/// 解析键值对用于表单数据
pub fn parse_form_kv_pair(s: &str) -> Result<crate::shared::types::KvPair> {
    s.parse()
} 