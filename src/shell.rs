extern crate rusql;
#[cfg(feature = "shellnav")]
extern crate readline;

#[cfg(feature = "shellnav")]
use readline::readline;
use rusql::{rusql_exec, Rusql};

#[cfg(not(feature = "shellnav"))]
use std::io;

#[cfg(not(feature = "shellnav"))]
fn readline(prompt: &str) -> Option<String> {
    print!("{}", prompt);

    io::stdin().read_line().ok()
}

pub fn main() {
    let mut db = Rusql::new();
    loop {
        let input = readline("rusql> ").expect("Error reading line");

        match input.as_slice() {
            ".make_foo" => {
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
