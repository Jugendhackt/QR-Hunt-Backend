mod structs;

use structs::{QrCode, QR, User};
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use rusqlite::{Connection, Result, params};
use std::collections::HashMap;
use actix_web::error::UrlencodedError::ContentType;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .service(hash)
            .service(signup)
            .wrap(cors)
    })
        .workers(4)
        .bind(("10.20.6.182", 8080))?
    .run()
    .await
}

#[post("/found")]
async fn hash(qr : web::Json<QR>) -> impl Responder {
    println!("{}, {}, {}\n", qr.id, qr.uid, qr.hash);

    let con = Connection::open("../../../db/qr.db").unwrap();
    let exe = con.execute("INSERT INTO qr_code (?1, ?2, ?3)", params![qr.id, qr.uid, qr.hash]);
    HttpResponse::Ok().body("success")
}

#[post("/signup")]
async fn signup(user : web::Json<User>) -> impl Responder {
    println!("{}, {}", user.username, user.password_hash);

    let con = Connection::open("../../db/qr.db").unwrap();
    let exe = con.execute("INSERT INTO user (?1, ?2)", params![user.username, user.password_hash]);

    HttpResponse::Ok().body("success")
}


// /login post fuer session tokens oder so was