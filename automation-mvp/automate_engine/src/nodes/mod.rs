use serde_json::Value;
use async_trait::async_trait;
use anyhow::Result;

pub mod http_node;

#[async_trait]
pub trait Node: Send + Sync {
    async fn run(&self, config: &Value, input: Value) -> Result<Value>;
}

pub fn get_node_by_type(node_type: &str) -> Result<Box<dyn Node>> {
    match node_type {
        "http" => Ok(Box::new(http_node::HttpNode::new())),
        _ => anyhow::bail!("Unknown node type"),
    }
}
