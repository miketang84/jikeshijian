use axum::{
    extract::{Form, Query},
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
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
        .route("/form", get(show_form).post(accept_form));

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

// async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
//     tracing::debug!("form params {:?}", input);

//     Html("<h3>Form posted</h3>")
// }

async fn accept_form(Form(input): Form<Input>) {
    tracing::debug!("form params {:?}", input);
}
