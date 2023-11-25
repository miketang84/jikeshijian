use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    // response::{Html, IntoResponse},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::net::SocketAddr;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

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
        "host=localhost user=postgres dbname=todolist password=123456",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        // .route("/query", get(query))
        // .route("/query_from_db", get(query_from_db))
        // .route("/json", post(accept_json))
        .route("/todos", get(todos_index))
        .route("/todo/new", post(todo_create))
        .route("/todo/update", post(todo_update))
        .route("/todo/delete/:id", post(todo_delete))
        .with_state(pool);

    let app = app.fallback(handler_404);

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: String,
    description: String,
    completed: bool,
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(pool): State<ConnectionPool>,
) -> Result<Json<Vec<Todo>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let Query(pagination) = pagination.unwrap_or_default();
    let offset: i64 = pagination.offset.unwrap_or(0);
    let limit: i64 = pagination.limit.unwrap_or(100);

    let rows = conn
        .query(
            "select id, description, completed from todo offset $1 limit $2",
            &[&offset, &limit],
        )
        .await
        .map_err(internal_error)?;

    let mut todos: Vec<Todo> = Vec::new();
    for row in rows {
        let id = row.get(0);
        let description = row.get(1);
        let completed = row.get(2);
        let todo = Todo {
            id,
            description,
            completed,
        };
        todos.push(todo);
    }

    Ok(Json(todos))
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    description: String,
}

async fn todo_create(
    State(pool): State<ConnectionPool>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, String)> {
    let todo = Todo {
        id: Uuid::new_v4().simple().to_string(),
        description: input.description,
        completed: false,
    };

    let conn = pool.get().await.map_err(internal_error)?;

    let _ret = conn
        .execute(
            "insert into todo (id, description, completed) values ($1, $2, $3) returning id",
            &[&todo.id, &todo.description, &todo.completed],
        )
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    id: String,
    description: Option<String>,
    completed: Option<bool>,
}

async fn todo_update(
    State(pool): State<ConnectionPool>,
    Json(utodo): Json<UpdateTodo>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let id: String = utodo.id.clone();
    let description: String = utodo.description.unwrap_or("".to_string());
    let completed: bool = utodo.completed.unwrap_or(false);
    tracing::debug!("id, desc, completed {} {} {}", id, description, completed);
    let _ret = conn
        .execute(
            "update todo set description=$1, completed=$2 where id=$3",
            &[&description, &completed, &id],
        )
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(id)))
}

async fn todo_delete(
    Path(id): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let _ret = conn
        .execute("delete from todo where id=$1", &[&id])
        .await
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(id)))
}
