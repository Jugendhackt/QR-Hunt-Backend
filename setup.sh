#!/bin/bash
echo -----------------------------[erstelle Datenbank]-----------------------------------------
cd db
cargo build
cargo run
echo -----------------------------[starte Webserver]-------------------------------------------

