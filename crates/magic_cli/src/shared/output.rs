use serde_json::Value;
use anyhow::Result;

/// 格式化JSON输出
pub fn format_json(value: &Value, compact: bool) -> Result<String> {
    if compact {
        Ok(serde_json::to_string(value)?)
    } else {
        Ok(serde_json::to_string_pretty(value)?)
    }
}

/// 打印HTTP响应信息
pub fn print_response_info(status: &reqwest::StatusCode, headers: &reqwest::header::HeaderMap) {
    println!("状态码: {}", status);
    println!("响应头: {:?}", headers);
} 