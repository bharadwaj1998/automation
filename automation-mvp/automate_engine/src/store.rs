use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{Value, json};
use std::path::Path;

const STORE_PATH: &str = "data/workflows.json";

pub fn init_store() {
    if !Path::new("data").exists() {
        std::fs::create_dir("data").unwrap();
    }

    if !Path::new(STORE_PATH).exists() {
        let f = File::create(STORE_PATH).unwrap();
        serde_json::to_writer_pretty(f, &json!([])).unwrap();
    }
}

pub fn load_all() -> Vec<Value> {
    let mut file = File::open(STORE_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap_or_default()
}

pub fn save_workflow(workflow: &Value) {
    let mut workflows = load_all();
    workflows.push(workflow.clone());

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(STORE_PATH)
        .unwrap();

    serde_json::to_writer_pretty(&mut file, &workflows).unwrap();
}
