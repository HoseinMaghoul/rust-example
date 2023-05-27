





use rusqlite::{Connection, Result};
use std::io;


#[derive(Debug)]
struct Hashkey {
    id: i32,
    hash_key: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE hashkey (
            id    INTEGER PRIMARY KEY,
            hash_key  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;




    let mut line = String::new();
    println!("Enter the hash key");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();




    let me = Hashkey {
        id: 1,
        hash_key: line.to_string(),
        data: None,
    };


    conn.execute(
        "INSERT INTO hashkey (hash_key, data) VALUES (?1, ?2)",
        (&me.hash_key, &me.data),
    )?;


    let mut stmt = conn.prepare("SELECT id, hash_key, data FROM hashkey")?;

    let hashkey_iter = stmt.query_map([], |row| {
        Ok(Hashkey {
            id: row.get(0)?,
            hash_key: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for hashkey in hashkey_iter {
        println!("Found person {:?}", hashkey.unwrap());
    }
   

  
    Ok(())
    

}