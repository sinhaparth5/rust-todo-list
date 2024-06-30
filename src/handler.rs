use actix_web::{HttpResponse, Responder};
use crate::models::Status;

pub(crate) async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status { status: "UP".to_string() })
}