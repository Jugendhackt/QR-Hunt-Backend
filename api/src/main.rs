mod structs;

use actix_web::{get, post, web, App, HttpServer, Responder};
use structs::QrCode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hash)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    let obj : QrCode = QrCode { id : 123, content : "Test".to_string() };
    web::Json(obj)
}

#[post("/")]
async fn hash(qr : web::Json<QrCode>) -> impl Responder {
    println!("{}\n", qr.id);
    qr
}