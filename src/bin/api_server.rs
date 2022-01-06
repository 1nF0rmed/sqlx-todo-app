extern crate sqlx_todo_app;

use actix_web::{middleware, App, HttpServer};
use sqlx_todo_app::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let serv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(routes)
    });
    serv.bind("0.0.0.0:8080")?.run().await
}
