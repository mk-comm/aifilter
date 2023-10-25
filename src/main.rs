use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde_json::json;
async fn index() -> impl IntoResponse {
    Json(json!({
    "message": "Hello World!"
    }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let port = match std::env::var("PORT") {
        Ok(val) => val,
        Err(_) => "8080".to_string(),
    };
    let addr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Failed to parse a port");
    let app = Router::new().route("/", get(index));

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start a server");
}
