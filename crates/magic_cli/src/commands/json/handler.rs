use std::io::{self, Read};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::commands::Command;
use crate::shared::format_json;
use super::{JsonArgs, query_json_path};

pub struct JsonCommand;

#[async_trait]
impl Command for JsonCommand {
    type Args = JsonArgs;

    async fn execute(&self, args: Self::Args) -> Result<()> {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let value: Value = serde_json::from_str(&input)?;

        let result_value = if let Some(query) = &args.query {
            // 简单的路径查询实现
            query_json_path(&value, query)?
        } else {
            value
        };

        let output = format_json(&result_value, args.compact)?;
        println!("{}", output);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "json"
    }

    fn description(&self) -> &'static str {
        "JSON处理工具"
    }
} 