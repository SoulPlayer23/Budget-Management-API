use actix_web::{delete, get, post, put, web::{self}, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::PgPool;
use time::{OffsetDateTime, PrimitiveDateTime};

//Model: Transaction - id, type, amount, date
#[derive(Serialize, Deserialize)]
struct Transaction {
    id: i32,
    purchase_type: String,
    amount: f64,
    userid: i32,
    #[serde(default = "defaultcurrenttime")]
    created_at: PrimitiveDateTime
}

//Funtion to provide current date and time
fn defaultcurrenttime() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now.date(), now.time())
}

//CRUD Operations

#[post("/transaction")]
async fn create_transaction(
    pool: web::Data<PgPool>, 
    transaction: web::Json<Transaction>
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO transaction (purchase_type, amount, userid, created_at) VALUES ($1, $2, $3, $4) RETURNING id, purchase_type, amount, userid, created_at",
        transaction.purchase_type,
        transaction.amount,
        transaction.userid,
        transaction.created_at
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


#[get("/transactions")]
async fn get_transactions(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM transaction")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(transactions) => {
            let formatted_transactions: Vec<serde_json::Value> = transactions
            .into_iter()
            .map(|transaction| {
                let created_at: PrimitiveDateTime = transaction.created_at.into();
                let formated_created_at = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}", 
                created_at.year(), created_at.month(), created_at.day(), created_at.hour(), created_at.minute(), created_at.second(), created_at.nanosecond());
                
                json!({
                    "id": transaction.id,
                    "purchase_type": transaction.purchase_type,
                    "amount": transaction.amount,
                    "userid": transaction.userid,
                    "created_at": formated_created_at
                })
            }).collect();
            HttpResponse::Ok().json(formatted_transactions)},
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/transaction/{id}")]
async fn get_transaction(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as!(Transaction, "SELECT * FROM transaction WHERE id = $1", *id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(transaction) => {
            let created_at: PrimitiveDateTime = transaction.created_at;
            let formated_created_at = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}", 
            created_at.year(), created_at.month(), created_at.day(), created_at.hour(), created_at.minute(), created_at.second(), created_at.nanosecond());
            HttpResponse::Ok().json(
                json!({
                    "id": transaction.id,
                    "purchase_type": transaction.purchase_type,
                    "amount": transaction.amount,
                    "userid": transaction.userid,
                    "created_at": formated_created_at
                })
            )
        },
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
    
#[put("/transaction/{id}")]
async fn update_transaction(pool: web::Data<PgPool>, id: web::Path<i32>, transaction: web::Json<Transaction>) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE transaction SET purchase_type = $1, amount = $2, userid = $3, created_at = $4 WHERE id = $5",
        transaction.purchase_type,
        transaction.amount,
        transaction.userid,
        transaction.created_at,
        *id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[delete("/transaction/{id}")]
async fn delete_transaction(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM transaction WHERE id = $1", *id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}