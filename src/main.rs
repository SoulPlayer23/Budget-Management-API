use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

//Load the modules - allows use of functions from the modules in main
mod transaction;
mod balance;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); //Load environment variables from .env file
    env_logger::init(); //Initialize logger

    //Establish database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    //Create new instance of the Actix web server, configuring with routes and service
    //web::scope('/api') - define API scope and register routes for all CRUD operations
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(
            web::scope("/api")
            .service(transaction::create_transaction)
            .service(transaction::get_transactions)
            .service(transaction::get_transaction)
            .service(transaction::update_transaction)
            .service(transaction::delete_transaction)
            .service(balance::create_balance)
            .service(balance::get_balance)
            .service(balance::update_balance)
            .service(balance::delete_balance)
        )
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address")
    .run()
    .await
}