import sqlite3
con = sqlite3.connect('database.db')
cur = con.cursor()
cur.execute("INSERT INTO QRcodes VALUES ('')")
