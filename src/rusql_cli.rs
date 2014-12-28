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

        match input.as_slice() {
            ".make_foo\n" => {
                rusql_exec(&mut db, "CREATE TABLE Foo(Id INTEGER, Name TEXT);
                                     INSERT INTO Foo VALUES
                                            (1, \"Foo1\"), (2, \"Foo2\"), (3, \"Foo3\");",
                           |_, _| ());
            }
            _ => {
                if let Some(results) = rusql_exec(&mut db, input.as_slice(), |_, _| {}) {
                    print!("{}", results);
                }
            }
        }
    }
}
