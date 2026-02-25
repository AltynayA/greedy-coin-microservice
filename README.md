# Greedy Coin Change Microservice

A REST microservice built with Rust and Axum that calculates optimal coin change for a given amount using the greedy algorithm.

## Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| GET | `/` | Service info |
| GET | `/health` | Health check |
| GET | `/change/:dollars/:cents` | Calculate coin change |

**Example:**
```bash
curl http://localhost:3000/change/1/75
```
```json
{"dollars": 1, "cents": 75, "change": [25, 25, 25, 25, 25, 25, 25]}
```

## Run Locally
```bash
cargo run
```

## Run with Docker
```bash
docker build -t greedy-coin-microservice .
docker run -dp 3000:3000 greedy-coin-microservice
```

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) — web framework
- [Tracing](https://github.com/tokio-rs/tracing) — structured JSON logging
- [rust_decimal](https://github.com/paupino/rust-decimal) — precise decimal arithmetic