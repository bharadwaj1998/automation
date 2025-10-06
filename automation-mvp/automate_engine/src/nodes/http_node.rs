use super::Node;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub struct HttpNode;

impl HttpNode {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Node for HttpNode {
    async fn run(&self, config: &Value, _input: Value) -> Result<Value> {
        let url = config["url"].as_str().unwrap_or("");
        let client = reqwest::Client::new();

        let mut req = client.get(url);
        if let Some(headers) = config["headers"].as_object() {
            for (k, v) in headers {
                if let Some(s) = v.as_str() {
                    req = req.header(k, s);
                }
            }
        }

        let res = req.send().await?;
        let status = res.status().as_u16();
        let text = res.text().await?;
        Ok(serde_json::json!({ "status": status, "body": text }))
    }
}
