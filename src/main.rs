use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod database;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = "postgres";
    let result = database::Database::new(conn.to_string());
    let db = match result {
        Err(e) => return Err(e),
        Ok(f) => f,
    };
    HttpServer::new(|| {
        App::new()
            .route("/add/{ticker}", web::post().to(crate::handlers::add_ticker(conn.to_string(), db)))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}