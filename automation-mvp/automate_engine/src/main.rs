use axum::{
    body::Body,
    extract::{Json, Path, State},
    http::{HeaderValue, Method, Request, Response, StatusCode},
    middleware::{self, Next},
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use chrono::Utc;
use async_stream::stream;

mod engine;
mod nodes;

#[derive(Clone, Default)]
struct AppState {
    runs: Arc<Mutex<Vec<Value>>>,
}

// ✅ Universal CORS middleware (fully preflight-safe)
async fn cors_layer(req: Request<Body>, next: Next) -> Response<Body> {
    // Handle preflight (OPTIONS)
    if req.method() == Method::OPTIONS {
        let mut res = Response::new(Body::empty());
        *res.status_mut() = StatusCode::NO_CONTENT;

        let headers = res.headers_mut();
        headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
        headers.insert(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
        );
        headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));

        return res;
    }

    // For normal requests
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    headers.insert("Access-Control-Allow-Headers", HeaderValue::from_static("*"));
    res
}

#[tokio::main]
async fn main() {
    let state = AppState::default();

    let app = Router::new()
        .route("/workflow", post(save_workflow))
        .route("/workflows", get(list_workflows))
        .route("/run/:id", post(run_workflow_by_id))
        .route("/runs", get(get_runs))
        .route("/events", get(sse_events))
        .layer(middleware::from_fn(cors_layer)) // ✅ Global CORS layer
        .with_state(state.clone());

    println!("🚀 Server running at http://0.0.0.0:3000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

// 🧩 Save workflow (POST /workflow)
async fn save_workflow(Json(payload): Json<Value>) -> Json<Value> {
    std::fs::create_dir_all("data").unwrap();
    let path = "data/workflows.json";

    let mut workflows: Vec<Value> = if std::path::Path::new(path).exists() {
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap_or_default()
    } else {
        vec![]
    };

    workflows.push(payload.clone());
    std::fs::write(path, serde_json::to_string_pretty(&workflows).unwrap()).unwrap();

    Json(json!({ "message": "Workflow saved successfully" }))
}

// 🧩 List all workflows (GET /workflows)
async fn list_workflows() -> Json<Value> {
    let workflows: Vec<Value> = if std::path::Path::new("data/workflows.json").exists() {
        serde_json::from_str(&std::fs::read_to_string("data/workflows.json").unwrap()).unwrap()
    } else {
        vec![]
    };
    Json(json!({ "workflows": workflows }))
}

// 🧩 Get workflow runs (GET /runs)
async fn get_runs(State(state): State<AppState>) -> Json<Value> {
    let runs = state.runs.lock().unwrap().clone();
    Json(json!({ "runs": runs }))
}

// 🧩 Run a workflow by ID (POST /run/:id)
async fn run_workflow_by_id(Path(id): Path<usize>, State(state): State<AppState>) -> Json<Value> {
    let workflows: Vec<Value> = if std::path::Path::new("data/workflows.json").exists() {
        serde_json::from_str(&std::fs::read_to_string("data/workflows.json").unwrap()).unwrap()
    } else {
        vec![]
    };

    if let Some(flow) = workflows.get(id) {
        println!("⚙️ Running workflow {id}");
        let result = engine::run_workflow(flow.clone()).await;

        let log = json!({
            "workflow_id": id,
            "timestamp": Utc::now().to_rfc3339(),
            "status": "success",
            "result": result
        });

        let mut runs = state.runs.lock().unwrap();
        runs.push(log.clone());

        Json(json!({ "message": "Workflow executed", "log": log }))
    } else {
        Json(json!({ "error": "Workflow not found" }))
    }
}

// 🧩 Live SSE Events (GET /events)
async fn sse_events(State(state): State<AppState>) -> impl IntoResponse {
    let stream = stream! {
        loop {
            let data = {
                let runs = state.runs.lock().unwrap();
                serde_json::to_string(&*runs).unwrap()
            };
            yield Ok::<_, axum::Error>(Event::default().data(data));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    Sse::new(stream)
}
