use axum::{extract::{Path, State}, routing::{get, post, delete, put}, Json, Router, http::StatusCode};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

#[derive(Deserialize)]
#[derive(Serialize, FromRow)]
struct Item { id: i32, name: String, price: rust_decimal::Decimal }

#[derive(Serialize, FromRow)]
#[derive(Deserialize)]
struct ItemPayLoad { name: String, price: rust_decimal::Decimal }

async fn get_item(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Item>, StatusCode> {
    sqlx::query_as::<_,Item>("SELECT id, name, price FROM items where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}
async fn create_item(State(pool): State<PgPool>,Json(item):Json<ItemPayLoad>) -> Result<Json<Item>,StatusCode>{
    sqlx::query_as::<_,Item>("INSERT into items(name, price) VALUES ($1,$2) RETURNING id, name, price")
        .bind(&item.name)
        .bind(item.price)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)


}

async fn update_item(State(pool): State<PgPool>,Path(id):Path<i32>, Json(item):Json<Item>) -> Result<StatusCode,StatusCode>{
    sqlx::query("UPDATE items SET name = $1, price = $2 WHERE id = $3")
        .bind(&item.name)
        .bind(item.price)
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)


}
async fn delete_item(State(pool): State<PgPool>,Path(id): Path<i32>) -> Result<StatusCode,StatusCode>{
    sqlx::query("DELETE FROM items WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)


}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://bench:bench@localhost:5432/bench".to_string());

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new().route("/items/{id}", get(get_item))
                           .route("/items", post(create_item))
                           .route("/items/{id}", put(update_item))
                           .route("/items/{id}", delete(delete_item))
                           .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

