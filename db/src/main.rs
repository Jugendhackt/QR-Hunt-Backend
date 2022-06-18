use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
    

fn main() -> Result<()> {
    let conn = Connection::open("qr.db")?;

    conn.execute(
        "create table if not exists qr_code (
             id integer primary key,
             content text not null unique,
             qrhash text not null unique,
             kordinaten text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists user (
             uuid integer primary key,
             qrcode_id integer not null references qr_code(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}