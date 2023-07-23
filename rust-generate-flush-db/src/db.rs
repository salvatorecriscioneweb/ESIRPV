use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use rusqlite::{Connection, MappedRows, Result};
use sha2::{Digest, Sha256};
use std::{
    borrow::BorrowMut,
    fs,
    io::{BufReader, Read},
};

use crate::document::Document;

pub struct DB {
    path_location: String,
}

const NO_PARAMS: () = ();

impl DB {
    fn open_db(&self, path_location: String) -> Result<Connection> {
        let conn = Connection::open(path_location.clone())?;
        Ok(conn)
    }
    pub fn run(path_location: String) -> Self {
        Self { path_location }
    }
    pub fn create_tables(&mut self) -> Result<()> {
        self.create_table_orders()?;
        self.create_table_products()?;
        self.create_table_documents()?;
        Ok(())
    }
    fn create_table_orders(&mut self) -> Result<()> {
        let mut conn: Connection = self.open_db(self.path_location.clone())?;
        let tx = conn.transaction()?;
        tx.execute(
            "create table if not exists orders (
             id integer primary key,
             number text not null unique,
             products text,
             sender text not null,
             receiver text not null,
             timestamp text not null
         )",
            NO_PARAMS,
        )?;
        tx.commit()
    }
    fn create_table_products(&mut self) -> Result<()> {
        let mut conn: Connection = self.open_db(self.path_location.clone())?;
        let tx = conn.transaction()?;
        tx.execute(
            "create table if not exists products (
             id integer primary key,
             sku text not null unique,
             img text,
             name text not null
         )",
            NO_PARAMS,
        )?;
        tx.commit()
    }
    fn create_table_documents(&mut self) -> Result<()> {
        let mut conn: Connection = self.open_db(self.path_location.clone())?;
        let tx = conn.transaction()?;
        tx.execute(
            "create table if not exists documents (
             id text primary key,
             file text not null unique,
             hash text not null unique,
             timestamp text not null
         )",
            NO_PARAMS,
        )?;
        tx.commit()
    }
    pub fn digest_file(&self, path: &String) -> Result<()> {
        self.digest(path)?;
        Ok(())
    }
    fn digest(&self, path: &String) -> Result<()> {
        let mut conn: Connection = self.open_db(self.path_location.clone())?;
        let tx = conn.transaction()?;
        let input = match fs::File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("file not found"),
        };
        let mut reader = BufReader::new(input);

        let digest = {
            let mut hasher = Sha256::new();
            let mut buffer = [0; 2048];
            loop {
                let count = match reader.read(&mut buffer) {
                    Ok(s) => s,
                    Err(_) => panic!("Error reading"),
                };
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            hasher.finalize()
        };
        let str_hash = format!("{:X}", digest);
        println!("Digest calculated {}", str_hash);
        let now = Utc::now();
        tx.execute(
            "INSERT INTO documents (id, file, hash, timestamp) VALUES (
                lower(hex(randomblob(10))), ?1, ?2, ?3)",
            (path, str_hash, now.to_rfc3339()),
        )?;
        tx.commit()
    }
    pub fn flush(&self) -> Result<()> {
        let conn: Connection = self.open_db(self.path_location.clone())?;

        let mut items: Vec<Document> = vec![];
        let mut stmt = conn.prepare("SELECT id, file, hash, timestamp FROM documents")?;
        let documents_iter = stmt.query_map([], |row| {
            Ok(Document {
                id: row.get(0)?,
                file: row.get(1)?,
                hash: row.get(2)?,
                timestamp: row.get(3)?,
            })
        })?;

        for document in documents_iter {
            let d = document.unwrap();
            items.push(d);
        }
        Some(self.flush_db(items));
        Ok(())
    }
    fn flush_db(&self, documents: Vec<Document>) -> Result<()> {
        println!("Start flush {}", documents.len());
        let mut conn: Connection = self.open_db(self.path_location.clone())?;
        let tx = conn.transaction()?;
        let now = Utc::now();
        for f in documents {
            println!("found hash {:?} {:?}", f.hash, f.timestamp);
            let datetime = DateTime::parse_from_rfc3339(f.timestamp.as_str()).unwrap();
            if now.timestamp() <= datetime.timestamp() + (60 * 5) {
                println!(
                    "Delete hash {:?} because {:?} {:?}",
                    f.hash,
                    now.timestamp(),
                    datetime.timestamp()
                );
                tx.execute(
                    "
                    delete from documents
                    where id = ?1
                    ",
                    [f.id],
                )?;
            }
        }
        tx.commit()
    }
}
