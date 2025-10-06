use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{Value, json};
use std::path::Path;
use chrono::Utc;

const LOG_PATH: &str = "data/runs.json";

pub fn init_logs() {
    if !Path::new("data").exists() {
        std::fs::create_dir("data").unwrap();
    }

    if !Path::new(LOG_PATH).exists() {
        let f = File::create(LOG_PATH).unwrap();
        serde_json::to_writer_pretty(f, &json!([])).unwrap();
    }
}

pub fn load_logs() -> Vec<Value> {
    let mut file = File::open(LOG_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap_or_default()
}

pub fn append_log(workflow_id: usize, status: &str, result: &Value) {
    let mut logs = load_logs();
    let entry = json!({
        "workflow_id": workflow_id,
        "timestamp": Utc::now().to_rfc3339(),
        "status": status,
        "result": result
    });
    logs.push(entry);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(LOG_PATH)
        .unwrap();
    serde_json::to_writer_pretty(&mut file, &logs).unwrap();
}
