use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handler::status;

mod models;
mod config;
mod handler;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();
    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .data(pool.close())
            .route("/", web::get().to(status))
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}