use serde_json::{json, Value};
use reqwest::Client;

/// Run a workflow by iterating through its nodes
pub async fn run_workflow(workflow_data: Value) -> Value {
    let client = Client::new();

    if let Some(nodes) = workflow_data.get("nodes").and_then(|n| n.as_array()) {
        let mut results = vec![];

        for node in nodes {
            let node_type = node["type"].as_str().unwrap_or("");
            match node_type {
                "http" => {
                    let url = node["config"]["url"].as_str().unwrap_or("");
                    let method = node["config"]["method"]
                        .as_str()
                        .unwrap_or("GET")
                        .to_uppercase();

                    println!("🌐 Running HTTP Node: {} {}", method, url);

                    let response = match method.as_str() {
                        "POST" => client.post(url).send().await,
                        "PUT" => client.put(url).send().await,
                        "DELETE" => client.delete(url).send().await,
                        _ => client.get(url).send().await,
                    };

                    match response {
                        Ok(resp) => {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();
                            results.push(json!({
                                "type": "http",
                                "url": url,
                                "status": status.as_u16(),
                                "body": text,
                            }));
                        }
                        Err(e) => {
                            results.push(json!({
                                "type": "http",
                                "url": url,
                                "error": e.to_string(),
                            }));
                        }
                    }
                }
                _ => results.push(json!({
                    "type": node_type,
                    "status": "skipped"
                })),
            }
        }

        json!({
            "status": "completed",
            "results": results
        })
    } else {
        json!({ "error": "No nodes found" })
    }
}
