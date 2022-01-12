use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::database::Database;

pub async fn add_ticker(ticker: String, mut database: Database) -> impl Responder {
    database.add_ticker(ticker);
    return HttpResponse::Ok();
}

pub async fn remove_ticker(ticker: String, mut database: Database) -> impl Responder {
    database.remove_ticker(ticker);
    return HttpResponse::Ok();
}