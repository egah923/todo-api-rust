use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Serialize, FromRow)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    completed: Option<bool>,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}


// =========================
// MAIN FUNCTION
// =========================

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let state = AppState { db };

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo)
                .put(update_todo)
                .delete(delete_todo),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}


// =========================
// GET ALL TODOS
// =========================

async fn get_todos(
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, StatusCode> {

    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos ORDER BY id"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

// create_todo()

async fn create_todo(
    State(state): State<AppState>,
    Json(data): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {

    if data.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let completed = data.completed.unwrap_or(false);

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, completed)
        VALUES ($1, $2)
        RETURNING id, title, completed
        "#
    )
    .bind(data.title)
    .bind(completed)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}


// get_todo()

async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {

    let todo = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}


// update_todo()

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {

    let existing = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed FROM todos WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(todo) => todo,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let title = data.title.unwrap_or(existing.title);
    let completed = data.completed.unwrap_or(existing.completed);

    if title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET title = $1, completed = $2
        WHERE id = $3
        RETURNING id, title, completed
        "#
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todo))
}


// delete_todo()
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {

    let result = sqlx::query(
        "DELETE FROM todos WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

