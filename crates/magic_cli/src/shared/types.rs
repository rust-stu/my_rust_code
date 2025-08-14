use std::str::FromStr;
use anyhow::{anyhow, Result};

/// 键值对结构体
#[derive(Debug, PartialEq, Clone)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("解析键值对失败: {}", s));
        Ok(Self {
            k: split.next().ok_or_else(err)?.to_string(),
            v: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

/// HTTP请求体数据类型
#[derive(Debug)]
pub enum BodyData {
    Json(String),
    Form(Vec<KvPair>),
    Raw(String),
}

impl BodyData {
    pub fn content_type(&self) -> &str {
        match self {
            BodyData::Json(_) => "application/json",
            BodyData::Form(_) => "application/x-www-form-urlencoded",
            BodyData::Raw(_) => "text/plain",
        }
    }
    
    pub fn to_string(&self) -> String {
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