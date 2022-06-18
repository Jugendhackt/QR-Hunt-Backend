mod structs;

use actix_web::{get, post, web, App, HttpServer, Responder};
use actix_cors::Cors;
use structs::QrCode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
           //.allowed_origin("10.20.4.14")
           //.allowed_methods(vec!["GET", "POST"])
           //.allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
           //.allowed_header(header::CONTENT_TYPE);
        App::new()
            .service(index)
            .service(hash)
            .wrap(cors)
    })
        .workers(4)
        .bind(("10.20.6.182", 8080))?
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
    println!("{}, {}\n", qr.id, qr.content);

    qr
    // access-conrol-allow-origin: 10.20.4.14
}