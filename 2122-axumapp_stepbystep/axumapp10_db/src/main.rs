use axum::{
    extract::{Form, Json, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    foo: Option<i32>,
    bar: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres dbname=postgres password=123456",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(handler))
        .route("/query", get(query))
        .route("/query_from_db", get(query_from_db))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json))
        .with_state(pool);

    let app = app.fallback(handler_404);

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

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn query_from_db(State(pool): State<ConnectionPool>) -> Result<String, (StatusCode, String)> {
    tracing::debug!("get db conn {:?}", pool);
    let conn = pool.get().await.map_err(internal_error)?;

    tracing::debug!("query_from_db: 1");
    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    tracing::debug!("query_from_db: 2");

    let two: i32 = row.try_get(0).map_err(internal_error)?;
    tracing::debug!("query_from_db: 3");
    tracing::debug!("calc_result {:?}", two);

    Ok(two.to_string())
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
