use axum::{
    extract::{Form, Json, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tower_http::trace::TraceLayer;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json))
        .route("/resjson", post(res_json))
        .route("/resjson2", post(res_json2))
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn query(Query(params): Query<Params>) -> Html<&'static str> {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);

    Html("<h3>Form posted</h3>")
}

async fn accept_json(Json(input): Json<Input>) -> Html<&'static str> {
    tracing::debug!("json params {:?}", input);
    Html("<h3>Json posted</h3>")
}

async fn res_json(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
    Json(json!({
        "result": "ok",
        "number": 1,
    }))
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
struct Output {
    name: String,
    age: u32,
}

async fn res_json2(Json(input): Json<Input>) -> impl IntoResponse {
    tracing::debug!("json params {:?}", input);
   
    let a = Output {
        name: "mike".to_string(),
        age: 20,
    };
    Json(serde_json::to_value(a).unwrap())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
