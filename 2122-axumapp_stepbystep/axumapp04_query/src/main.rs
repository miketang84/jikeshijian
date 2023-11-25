use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct Params {
//     foo: Option<i32>,
//     bar: Option<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: i32,
    bar: String,
    aa: Option<i32>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}
