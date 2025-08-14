use anyhow::{anyhow, Result};
use serde_json::Value;

/// 根据路径查询JSON值
pub fn query_json_path(value: &Value, path: &str) -> Result<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;
    
    for part in parts {
        match current {
            Value::Object(obj) => {
                current = obj.get(part)
                    .ok_or_else(|| anyhow!("键 '{}' 未找到", part))?;
            }
            Value::Array(arr) => {
                let index: usize = part.parse()
                    .map_err(|_| anyhow!("无效的数组索引: {}", part))?;
                current = arr.get(index)
                    .ok_or_else(|| anyhow!("数组索引 {} 超出范围", index))?;
            }
            _ => return Err(anyhow!("无法在非对象/非数组上访问 '{}'", part)),
        }
    }
    
    Ok(current.clone())
} 