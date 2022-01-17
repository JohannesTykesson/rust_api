use actix_web::{web, HttpResponse, Responder};
use crate::database::Database;
use std::io;

pub struct Handler {
    db2: Database
}

impl Handler {
    pub fn new(database: Database) -> Result<Handler, io::Error> {
        let handler = Handler {
            db2: database
        };
        return Ok(handler);
    }

    pub async fn add_ticker(mut self, web::Path((ticker,)): web::Path<(String,)>) -> impl Responder {
        self.db2.add_ticker(ticker);
        return HttpResponse::Ok();
    }

    pub async fn remove_ticker(mut self, web::Path((ticker,)): web::Path<(String,)>) -> impl Responder {
        self.db2.remove_ticker(ticker);
        return HttpResponse::Ok();
    }
}