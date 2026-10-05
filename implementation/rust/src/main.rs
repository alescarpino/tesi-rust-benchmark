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

#[derive(Serialize, FromRow)]
struct BranchItemRevenue {
    branch_name: String,
    item_name: String,
    total_revenue: rust_decimal::Decimal,
    line_count: i64,
}

async fn revenue_by_branch(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<BranchItemRevenue>>, StatusCode> {
    sqlx::query_as::<_, BranchItemRevenue>(
        "SELECT
            b.name AS branch_name,
            i.name AS item_name,
            SUM(cr.quantity * cr.price) AS total_revenue,
            COUNT(*) AS line_count
        FROM contract_rows cr
        JOIN contracts c ON cr.contract_id = c.id
        JOIN branches b ON c.branch_id = b.id
        JOIN items i ON cr.item_id = i.id
        WHERE c.status IN ('approved', 'ongoing')
          AND c.start_date BETWEEN '2025-01-01' AND '2025-12-31'
        GROUP BY b.name, i.name
        ORDER BY total_revenue DESC
        LIMIT 50",
    )
    .fetch_all(&pool)
    .await
    .map(Json)
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
                           .route("/reports/revenue-by-branch", get(revenue_by_branch))
                           .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

