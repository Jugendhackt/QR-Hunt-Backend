use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QrCode {
   pub id : i32,
   pub content : String,
}