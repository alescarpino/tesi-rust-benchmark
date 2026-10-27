use axum::{extract::{Path, State}, routing::{get, post, delete, put}, Json, Router, http::StatusCode};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

#[derive(Deserialize)]
#[derive(Serialize, FromRow)]
struct Item { id: i32, name: String, price: rust_decimal::Decimal }

#[derive(Serialize, FromRow)]
#[derive(Deserialize)]
struct ItemPayLoad { name: String, price: rust_decimal::Decimal }


//TEST 1 CRUD //
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

// TEST 2 QUERY PESANTE//
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

// TEST 3 SERIALIZZAZIONE E DESERIALIZZAZIONE JSON
// GET  /reports/contract-rows-detailed  
// POST /contract-rows/validate 
#[derive(FromRow)]
struct ContractRowFlat {
    contract_id: i32,
    branch_id: i32,
    branch_name: String,
    item_id: i32,
    item_name: String,
    quantity: i32,
    price: rust_decimal::Decimal,
    line_number: i64,
}

#[derive(Serialize)]
struct BranchRef {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct ItemRef {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct ContractRowDetailed {
    contract_id: i32,
    branch: BranchRef,
    item: ItemRef,
    quantity: i32,
    price: rust_decimal::Decimal,
    line_number: i64,
}

async fn contract_rows_detailed(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ContractRowDetailed>>, StatusCode> {
    let flat_rows = sqlx::query_as::<_, ContractRowFlat>(
        "SELECT
            cr.contract_id,
            b.id AS branch_id,
            b.name AS branch_name,
            i.id AS item_id,
            i.name AS item_name,
            cr.quantity,
            cr.price,
            ROW_NUMBER() OVER (ORDER BY cr.id) AS line_number
        FROM contract_rows cr
        JOIN contracts c ON cr.contract_id = c.id
        JOIN branches b ON c.branch_id = b.id
        JOIN items i ON cr.item_id = i.id
        ORDER BY cr.id
        LIMIT 1000",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let detailed: Vec<ContractRowDetailed> = flat_rows
        .into_iter()
        .map(|r| ContractRowDetailed {
            contract_id: r.contract_id,
            branch: BranchRef { id: r.branch_id, name: r.branch_name },
            item: ItemRef { id: r.item_id, name: r.item_name },
            quantity: r.quantity,
            price: r.price,
            line_number: r.line_number,
        })
        .collect();

    Ok(Json(detailed))
}

#[derive(Deserialize)]
struct ContractRowInput {
    item_id: i32,
    quantity: i32,
    price: rust_decimal::Decimal,
}

#[derive(Deserialize)]
struct ValidateRowsPayload {
    rows: Vec<ContractRowInput>,
}

#[derive(Serialize)]
struct ValidateRowsResponse {
    received: usize,
    valid: usize,
    invalid: usize,
}

async fn validate_contract_rows(
    Json(payload): Json<ValidateRowsPayload>,
) -> Json<ValidateRowsResponse> {
    let received = payload.rows.len();
    let valid = payload
        .rows
        .iter()
        .filter(|r| r.item_id > 0 && r.quantity > 0 && r.price >= rust_decimal::Decimal::ZERO)
        .count();

    Json(ValidateRowsResponse {
        received,
        valid,
        invalid: received - valid,
    })
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
                           .route("/reports/contract-rows-detailed", get(contract_rows_detailed))
                           .route("/contract-rows/validate", post(validate_contract_rows))
                           .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

