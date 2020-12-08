use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::mysql::MySqlPool;
use std::env;

mod controller;
mod model;
mod router;
mod common;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = MySqlPool::connect(&database_url).await?;

    let pool = web::Data::new(db_pool);

    let mut server = HttpServer::new(move || {
        App::new().service(
            web::scope(&env::var("CONTENT_PATH").expect(""))
                .app_data(pool.clone())
                .configure(router::init),
        )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("127.0.0.1");
            let port = env::var("PORT").expect("6789");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await?;

    Ok(())
}
