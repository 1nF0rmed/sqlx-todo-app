extern crate sqlx_todo_app;

use std::env;
use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer};
use sqlx::PgPool;
use sqlx_todo_app::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&database_url).await.expect("Should connect to db");

    let serv = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .configure(routes)
    });
    serv.bind("0.0.0.0:8080")?.run().await
}
