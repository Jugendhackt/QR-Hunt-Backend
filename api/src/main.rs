mod structs;

use std::{fs, env};
use rand;
use structs::{QrCode, QR, User};
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use rusqlite::{Connection, Result, params, NO_PARAMS};
use std::collections::HashMap;
use actix_web::error::UrlencodedError::ContentType;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let con = Connection::open("database.db").unwrap();

    con.execute("CREATE TABLE if not exists Users (\
        id      INTEGER PRIMARY KEY ,\
        name    TEXT NOT NULL,\
        pwhash  TEXT NOT NULL );", NO_PARAMS).unwrap();
    con.execute("CREATE TABLE if not exists Lookup (\
        id      INTEGER PRIMARY KEY AUTOINCREMENT,\
        hash    TEXT NOT NULL UNIQUE,\
        content TEXT NOT NULL UNIQUE);", NO_PARAMS).unwrap();
    con.execute("CREATE TABLE if not exists QrCodes (\
        id      INTEGER NOT NULL,\
        userid  INTEGER NOT NULL,\
        hash    TEXT NOT NULL);", NO_PARAMS).unwrap();
    con.close();

    //fs::write("/tmp/current_id", start_value.to_string()).expect("writing to file went wrong");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            .service(hash)
            .service(signup)
            .service(login)
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

    let mut con = Connection::open("database.db").unwrap();

    let tx = con.transaction.unwrap();
    tx.execute("INSERT INTO qr_code values (?1, ?2, ?3)", params![qr.id, qr.uid, qr.hash]);
    tx.commit();
    con.close();

    HttpResponse::Ok().body("success")
}

#[post("/signup")]
async fn signup(user : web::Json<User>) -> impl Responder {
    println!("{}, {}", user.username, user.password_hash);
    let mut curr_id : i32 = fs::read_to_string("/tmp/current_id").unwrap().to_string().parse().unwrap();

    curr_id += 1;
    fs::write("/tmp/current_id", curr_id.to_string()).expect("writing to file went wrong");

    let mut con = Connection::open("database.db").unwrap();
    let tx = con.transaction().unwrap();

    tx.execute(format!("INSERT INTO Users VALUES ({}, \"{}\", \"{}\")", curr_id, user.username, user.password_hash).as_str(), NO_PARAMS);
    tx.commit();

    println!("{}", format!("INSERT INTO Users values (name, pwhash) VALUES (\"{}\", \"{}\", \"{}\")", 12, user.username, user.password_hash).as_str());
    con.close();
    web::Json("success")
}

#[post("/login")]
async fn login(user : web::Json<User>) -> impl Responder {
    println!("random 64-bit integer will be returned");
    web::Json(structs::ReturnToken { token: rand::random::<i64>() })
}