use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Error;
use crate::handlers::Handler;

mod database;
mod handlers;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    if let Ok(db) = database::Database::new(String::from("Conn")) {
        let handler = Handler {
            db2: db
        };
        HttpServer::new(|| {
            App::new()
            .route("/addticker/{ticker}", web::post().to(handler.add_ticker()))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await;
    } else {
        print!("Could not initiate database");
    }
    return Ok(());
}
