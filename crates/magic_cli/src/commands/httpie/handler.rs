use std::str::FromStr;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Method;

use crate::commands::Command;
use crate::shared::{BodyData, create_http_client, print_response_info};
use super::{HttpieArgs, HttpieSubCommand};

pub struct HttpieCommand;

#[async_trait]
impl Command for HttpieCommand {
    type Args = HttpieArgs;

    async fn execute(&self, args: Self::Args) -> Result<()> {
        let client = create_http_client()?;
        
        match args.subcmd {
            HttpieSubCommand::Get(get_args) => {
                println!("发送GET请求到: {}", get_args.url);
                self.send_request(&client, "GET", &get_args.url, None).await?;
            }
            HttpieSubCommand::Post(post_args) => {
                println!("发送POST请求到: {}", post_args.url);
                
                let body_data = self.prepare_body_data(&post_args);
                
                if let Some(body) = &body_data {
                    println!("请求体: {:?}", body);
                    println!("Content-Type: {}", body.content_type());
                    println!("请求体字符串: {}", body.to_string());
                } else {
                    println!("没有提供请求体数据");
                }
                
                self.send_request(&client, "POST", &post_args.url, body_data).await?;
            }
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "httpie"
    }

    fn description(&self) -> &'static str {
        "HTTP客户端工具"
    }
}

impl HttpieCommand {
    async fn send_request(
        &self,
        client: &reqwest::Client,
        method: &str,
        url: &str,
        body: Option<BodyData>
    ) -> Result<()> {
        let mut req = client.request(Method::from_str(method)?, url);
        
        if let Some(body) = body {
            req = req.body(body.to_string());
        }
        
        let resp = req.send().await?;
        print_response_info(&resp.status(), resp.headers());
        
        let body = resp.text().await?;
        println!("响应体: {}", body);
        
        Ok(())
    }

    fn prepare_body_data(&self, args: &super::HttpiePostArgs) -> Option<BodyData> {
        if let Some(json) = &args.json {
            Some(BodyData::Json(json.clone()))
        } else if !args.form.is_empty() {
            Some(BodyData::Form(args.form.clone()))
        } else if let Some(data) = &args.data {
            Some(BodyData::Raw(data.clone()))
        } else {
            None
        }
    }
} 