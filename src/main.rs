use axum::extract::Path;
use axum::{routing::get, Json, Router};
use rust_axum_greedy_coin_microservice::greedy_coin_change;
use rust_decimal::Decimal;
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing_subscriber::fmt;

//Root Route for Change Machine
async fn root() -> &'static str {
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
    let change = greedy_coin_change(amount);

    let json = json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    });
    Json(json)
}

async fn health() -> impl axum::response::IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "greedy-coin-change"
    }))
}

#[tokio::main]
async fn main() {
    let filter = tracing_subscriber::EnvFilter::new("info,tower_http=debug");
    //  json logging in production
    //  pretty logging for local
    //  switch  with APP_ENV=production cargo run
    match std::env::var("APP_ENV").as_deref() {
        Ok("production") => fmt()
            .json()
            .with_target(true)
            .with_current_span(true)
            .with_env_filter(filter)
            .init(),

        _ => fmt()
            .pretty()
            .with_target(true)
            .with_env_filter(filter)
            .init(),
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/change/:dollars/:cents", get(change))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
