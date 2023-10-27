use axum::routing::get;
use axum::Router;

use crate::endpoints::index;

mod endpoints;

#[tokio::main]
async fn main() {
    //test

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
