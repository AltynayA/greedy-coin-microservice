use axum::extract::Path;
use axum::{routing::get, Json, Router};
use rust_axum_greedy_coin_microservice::greedy_coin_change;
use rust_decimal::Decimal;
use serde_json::json;
use tracing::{info, instrument};
use tracing_subscriber::fmt;

//Root Route for Change Machine
async fn root() -> &'static str {
    info!("Root endpoint called");
    "
    Greedy Coin Change Machine

    **Primary Route:**
    /change/dollars/cents
    "
}

#[instrument]
async fn change(Path((dollars, cents)): Path<(u32, u32)>) -> impl axum::response::IntoResponse {
    // Convert to cents amount
    let amount = Decimal::from(dollars * 100 + cents);
    info!(dollars, cents, "Processing change request");
    let change = greedy_coin_change(amount);
    info!(coin_count = change.len(), "Change calculated successfully");

    let json = json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    });
    Json(json)
}

async fn health() -> impl axum::response::IntoResponse {
    info!("Health check called");
    Json(json!({
        "status": "ok",
        "service": "greedy-coin-change"
    }))
}

#[tokio::main]
async fn main() {
    // initialize json logger
    fmt()
        .json()
        .with_target(true)
        .with_current_span(true)
        .init();

    info!("Starting Greedy Coin Change service on port 3000");
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/change/:dollars/:cents", get(change));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
