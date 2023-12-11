use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/greet/:name", get(greet))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    HelloTemplate { name } .to_string()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
