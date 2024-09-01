use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Balance {
    id: i32,
    userid: i32,
    balance: f64
}

//Create
#[post("/balance")]
async fn create_balance(pool: web::Data<PgPool>, balance: web::Json<Balance>) -> impl Responder {
    let result = sqlx::query!(
        r#"INSERT INTO balance (userid, balance) VALUES ($1, $2) RETURNING id, userid, balance"#, balance.userid, balance.balance
    )
    .fetch_one(pool.get_ref())
    .await;

    match result{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[get("/balance/{userid}")]
async fn get_balance(pool: web::Data<PgPool>, userid: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("SELECT * from balance WHERE userid = $1", *userid)
        .fetch_all(pool.get_ref())
        .await;

    match result{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[put("/balance/{userid}")]
async fn update_balance(pool: web::Data<PgPool>, userid: web::Path<i32>, balance: web::Json<Balance>) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE balance SET balance = $1 WHERE userid = $2",
        balance.balance,
        *userid
    )
    .execute(pool.get_ref())
    .await;

    match result{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

#[delete("/balance/{userid}")]
async fn delete_balance(pool: web::Data<PgPool>, userid: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM balance WHERE userid = $1", *userid)
        .execute(pool.get_ref())
        .await;

    match result{
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}