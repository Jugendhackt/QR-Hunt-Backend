use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QrCode {
   pub id : i32,
   pub hash : String,
   pub content : String,
   pub location : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QR {
   pub id : i32,
   pub uid : i32,
   pub hash : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
   pub username : String,
   pub password_hash : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnToken {
   pub token: i64
}

