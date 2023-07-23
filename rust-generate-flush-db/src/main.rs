use std::env;

use crate::db::DB;

use crate::errors::AppError;

mod db;
mod document;
mod errors;
mod order;
mod products;

fn main() {
    // rust-generate-flush-db flush db.sqlite
    let flush_cmd = "flush";
    // rust-generate-flush-db generate [order] [min_prod] [max_prod] db.sqlite
    let generate_cmd = "generate";
    // rust-generate-flush-db digest [file] db.sqlite
    let digest_cmd = "digest";

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        // Error params
        println!("{}", AppError::missing_cmd_env().to_string());
        return;
    }
    let cmd = args.get(1).unwrap();
    let db_path = args.last().unwrap();

    let mut db = DB::run(db_path.clone());
    let res = db.create_tables();
    match res {
        Ok(_) => {}
        Err(err) => panic!("tables creation failed {}", err),
    };
    println!("current cmd {} {}", cmd.to_owned(), digest_cmd);
    if cmd.eq_ignore_ascii_case(flush_cmd) {
        println!("FLUSH {}", db_path);
        match db.flush() {
            Ok(_) => println!("Flush ok"),
            Err(_) => panic!(),
        };
    }
    if cmd.eq_ignore_ascii_case(generate_cmd) {
        println!("GENERATE {}", db_path);
    }
    if cmd.eq_ignore_ascii_case(digest_cmd) {
        let file = args.get(2).unwrap();
        println!("DIGEST {} of file {}", db_path, file);
        Some(db.digest_file(file));
    }
}
