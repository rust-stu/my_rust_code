use anyhow::Result;

/// 验证JSON查询路径格式
pub fn validate_json_query(query: &str) -> Result<String> {
    // 基本的查询路径验证
    if query.is_empty() {
        return Err(anyhow::anyhow!("查询路径不能为空"));
    }
    
    // 检查是否包含无效字符
    if query.contains("..") {
        return Err(anyhow::anyhow!("查询路径不能包含连续的点"));
    }
    
    // 验证数组索引格式
    for part in query.split('.') {
        if part.parse::<usize>().is_err() && !is_valid_object_key(part) {
            return Err(anyhow::anyhow!("无效的查询路径部分: {}", part));
        }
    }
    
    Ok(query.to_string())
}

/// 检查是否为有效的对象键名
pub(super) fn is_valid_object_key(key: &str) -> bool {
    !key.is_empty() && 
    key.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
} 