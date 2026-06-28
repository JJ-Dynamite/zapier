use axum::{extract::{Path, Json, State}, routing::{get, post}, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
struct Integration { id: String, name: String, icon: String, category: String, connected: bool }

#[derive(Deserialize)]
struct ConnectRequest { app_id: String }

type Store = Arc<RwLock<Vec<Integration>>>;

async fn health() -> &'static str { "OK" }
async fn root() -> &'static str { "Zapier API - Wire any two apps together" }

async fn list_integrations(State(store): State<Store>) -> Json<Vec<Integration>> {
    let apps = store.read().await;
    Json(apps.clone())
}

async fn connect_app(State(store): State<Store>, Json(input): Json<ConnectRequest>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut apps = store.write().await;
    if let Some(app) = apps.iter_mut().find(|a| a.id == input.app_id) {
        app.connected = true;
        return Ok(Json(serde_json::json!({ "status": "connected", "app": app.name })));
    }
    Err(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    let integrations = vec![
        Integration { id: "slack".into(), name: "Slack".into(), icon: "💬".into(), category: "Communication".into(), connected: false },
        Integration { id: "gmail".into(), name: "Gmail".into(), icon: "📧".into(), category: "Email".into(), connected: false },
        Integration { id: "github".into(), name: "GitHub".into(), icon: "🐙".into(), category: "Development".into(), connected: false },
        Integration { id: "sheets".into(), name: "Google Sheets".into(), icon: "📊".into(), category: "Productivity".into(), connected: false },
        Integration { id: "trello".into(), name: "Trello".into(), icon: "📋".into(), category: "Project Management".into(), connected: false },
        Integration { id: "twitter".into(), name: "Twitter".into(), icon: "🐦".into(), category: "Social".into(), connected: false },
        Integration { id: "stripe".into(), name: "Stripe".into(), icon: "💳".into(), category: "Payments".into(), connected: false },
        Integration { id: "notion".into(), name: "Notion".into(), icon: "📝".into(), category: "Productivity".into(), connected: false },
    ];
    let store: Store = Arc::new(RwLock::new(integrations));
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/integrations", get(list_integrations))
        .route("/connect", post(connect_app))
        .with_state(store);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Zapier running on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
