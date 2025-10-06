use axum::{
    extract::{Json, Path, State},
    http::Method,
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use chrono::Utc;
use async_stream::stream;
use axum::body::Body;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use uuid::Uuid;

mod engine;
mod nodes;

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
    runs: Arc<Mutex<Vec<Value>>>,
}

#[tokio::main]
async fn main() {
    // ✅ Connect to Postgres
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:secret@localhost:5432/automate")
        .await
        .expect("❌ Failed to connect to Postgres");

    // ✅ Create each table separately to avoid multi-command error
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS workflows (
            id SERIAL PRIMARY KEY,
            user_id UUID NOT NULL,
            name TEXT NOT NULL,
            data JSONB NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        );
        "#
    )
    .execute(&db)
    .await
    .expect("❌ Failed to create 'workflows' table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS runs (
            id SERIAL PRIMARY KEY,
            workflow_id INT REFERENCES workflows(id) ON DELETE CASCADE,
            status TEXT NOT NULL,
            result JSONB,
            created_at TIMESTAMP DEFAULT NOW()
        );
        "#
    )
    .execute(&db)
    .await
    .expect("❌ Failed to create 'runs' table");

    let state = AppState {
        db,
        runs: Arc::new(Mutex::new(Vec::new())),
    };

    // ✅ CORS setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // ✅ Routes
    let app = Router::new()
        .route("/workflow", post(save_workflow))
        .route("/workflows", get(list_workflows))
        .route("/run/:id", post(run_workflow_by_id))
        .route("/runs", get(get_runs))
        .route("/events", get(sse_events))
        .layer(cors)
        .with_state(state.clone());

    println!("🚀 Server running at http://0.0.0.0:3000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

// ✅ Save a workflow into the DB
async fn save_workflow(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4();
    let name = payload["name"]
        .as_str()
        .unwrap_or("Untitled Workflow")
        .to_string();
    let data = serde_json::to_value(&payload).unwrap_or(json!({}));

    sqlx::query!(
        "INSERT INTO workflows (user_id, name, data) VALUES ($1, $2, $3)",
        user_id,
        name,
        data
    )
    .execute(&state.db)
    .await
    .expect("❌ Failed to insert workflow");

    Json(json!({ "message": "Workflow saved successfully" }))
}

// ✅ Fetch all workflows
async fn list_workflows(State(state): State<AppState>) -> impl IntoResponse {
    let rows = sqlx::query!(
        "SELECT id, name, data, created_at FROM workflows ORDER BY id DESC"
    )
    .fetch_all(&state.db)
    .await
    .expect("❌ Failed to fetch workflows");

    let workflows: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            let data = match r.data {
                Value::Null => json!({}),
                _ => r.data,
            };
            json!({
                "id": r.id,
                "name": r.name,
                "data": data,
                "created_at": r.created_at
            })
        })
        .collect();

    Json(json!({ "workflows": workflows }))
}

// ✅ Return all in-memory run logs
async fn get_runs(State(state): State<AppState>) -> impl IntoResponse {
    let runs = state.runs.lock().unwrap().clone();
    Json(json!({ "runs": runs }))
}

// ✅ Execute a workflow by its ID
async fn run_workflow_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let row = sqlx::query!("SELECT id, data FROM workflows WHERE id = $1", id)
        .fetch_one(&state.db)
        .await;

    match row {
        Ok(r) => {
            let data = match r.data {
                Value::Null => json!({}),
                _ => r.data,
            };
            println!("⚙️ Running workflow {}", id);

            let result = engine::run_workflow(data.clone()).await;

            sqlx::query!(
                "INSERT INTO runs (workflow_id, status, result) VALUES ($1, $2, $3)",
                id,
                "success",
                serde_json::to_value(&result).unwrap_or_default()
            )
            .execute(&state.db)
            .await
            .expect("❌ Failed to insert run");

            let log = json!({
                "workflow_id": id,
                "timestamp": Utc::now().to_rfc3339(),
                "status": "success",
                "result": result
            });

            let mut runs = state.runs.lock().unwrap();
            runs.push(log.clone());

            Json(json!({ "message": "Workflow executed", "log": log }))
        }
        Err(_) => Json(json!({ "error": "Workflow not found" })),
    }
}

// ✅ Real-time SSE for live updates
async fn sse_events(State(state): State<AppState>) -> impl IntoResponse {
    let stream = stream! {
        loop {
            let data = {
                let runs = state.runs.lock().unwrap();
                serde_json::to_string(&*runs).unwrap()
            };
            yield Ok::<_, axum::Error>(Event::default().data(data));
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    Sse::new(stream)
}
