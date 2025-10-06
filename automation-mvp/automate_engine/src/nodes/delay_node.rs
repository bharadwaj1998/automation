use crate::nodes::Node;
use async_trait::async_trait;
use serde_json::Value;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub struct DelayNode;

impl DelayNode {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl Node for DelayNode {
    async fn run(&self, config: &Value, input: Value) -> Result<Value> {
        let ms = config.get("ms").and_then(|v| v.as_u64()).unwrap_or(1000);
        sleep(Duration::from_millis(ms)).await;
        Ok(input)
    }
}
