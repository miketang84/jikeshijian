use axum::{
    extract::{Form, Json, Query},
    response::Html,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json));

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

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}

// #[derive(Debug, Serialize)]
// #[allow(dead_code)]
// struct Output {
//     name: String,
//     age: u32,
// }

async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);

    Html("<h3>Form posted</h3>")
}

// async fn accept_json(Json(input): Json<Input>) -> String {
//     tracing::debug!("json params {:?}", input);
//     // Json("{\"a\": \"1\"}")
//     json!({
//         "result": "ok",
//         "number": 1,
//     })
//     .to_string()
// }

async fn accept_json(Json(input): Json<Input>) -> Json<serde_json::Value> {
    tracing::debug!("json params {:?}", input);
    Json(json!({
        "result": "ok",
        "number": 1,
    }))

    // let a = Output {
    //     name: "mike".to_string(),
    //     age: 20,
    // };
    // Json(serde_json::to_value(a).unwrap())
}
