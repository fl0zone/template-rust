use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, Json, Router};
use serde_json::Value;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("could not parse PORT env var");
    println!("starting test service on port: {port}");

    let state = CountState {
        counter: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/", axum::routing::get(root))
        .with_state(state);

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root(state: State<CountState>) -> Json<Value> {
    let mut c = state.0.counter.lock().await;
    *c += 1;

    println!("counter: {0}", *c);

    Json(serde_json::json!({ "counter": *c }))
}

#[derive(Clone)]
struct CountState {
    counter: Arc<Mutex<usize>>,
}
