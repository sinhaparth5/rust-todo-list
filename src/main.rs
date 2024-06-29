mod models;

use axum::{
    routing::get,
    Router,
    Json,
};
use crate::models::Status;
use std::net::SocketAddr;
use axum_server::Server;

async fn status() -> Json<Status> {
    Json(Status { status: "Ok".to_string() })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(status));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Starting server at http://{}", addr);

    Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}