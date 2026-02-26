use axum::extract::Path;
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use rust_axum_greedy_coin_microservice::greedy_coin_change;
use rust_decimal::Decimal;
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing_subscriber::fmt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Listing all endpoints
#[derive(OpenApi)]
#[openapi(
    paths(change, health),
    info(
        title = "Greedy Coin Change API",
        version = "1.0.0",
        description = "A microservice that calculates optimal coin change using the greedy algorithm"
    )
)]
struct ApiDoc;

//Root Route for Change Machine
async fn root() -> &'static str {
    "
    Greedy Coin Change Machine

    **Primary Route:**
    /change/dollars/cents
    "
}
// Calculates greedy coin change for a given dollar and cent amount
#[utoipa::path(
    get,
    path = "/change/{dollars}/{cents}",
    params(
        ("dollars" = u32, Path, description = "Dollar amount"),
        ("cents" = u32, Path, description = "Cent amount (0-99)")
    ),
    responses(
        (status = 200, description = "Coin change calculated successfully"),
        (status = 400, description = "Invalid input — cents > 99 or amount is zero")
    )
)]
#[instrument]
async fn change(
    Path((dollars, cents)): Path<(u32, u32)>,
) -> Result<impl axum::response::IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Cents must be between 0 and 99
    if cents > 99 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "cents must be between 0 and 99",
                "received": cents
            })),
        ));
    }

    // Amount must be greater than zero
    if dollars == 0 && cents == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "amount must be greater than zero" })),
        ));
    }

    // Convert to cents amount
    let amount = Decimal::from(dollars * 100 + cents);
    let change = greedy_coin_change(amount);

    Ok(Json(json!({
        "dollars": dollars,
        "cents": cents,
        "change": change
    })))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
