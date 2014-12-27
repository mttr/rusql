extern crate rusql;

use rusql::{rusql_exec, Rusql};

use std::io;

pub fn main() {
    let mut db = Rusql::new();
    loop {
        print!("rusql> ");

        let input = io::stdin().read_line()
                               .ok()
                               .expect("Failed to read line");

        rusql_exec(&mut db, input.to_string(), |entry, _| {
            for column in entry.iter() {
                print!("{} | ", column);
            }
            print!("\n");
        });
    }
}
