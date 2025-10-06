use serde_json::Value;
use crate::nodes::{get_node_by_type, Node};
use anyhow::Result;

pub async fn run_workflow(workflow: Value) -> Value {
  let nodes_vec = workflow["nodes"].as_array().cloned().unwrap_or_default();

    let mut data = Value::Null;

    for node in nodes_vec {
    let node_type = node["type"].as_str().unwrap_or("unknown");
    let config = node["config"].clone();
    let node_impl = get_node_by_type(node_type).unwrap();

    match node_impl.run(&config, data.clone()).await {
        Ok(result) => data = result,
        Err(e) => return Value::String(format!("Node '{}' failed: {}", node_type, e)),
    }
}

    data
}
