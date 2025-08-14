// 通用验证器
// 具体的验证逻辑已经移到各个子命令模块中
// 这里只保留跨命令的通用验证功能

use anyhow::Result;

/// 通用的非空字符串验证
pub fn validate_non_empty_string(s: &str) -> Result<String> {
    if s.trim().is_empty() {
        return Err(anyhow::anyhow!("参数不能为空"));
    }
    Ok(s.to_string())
} 