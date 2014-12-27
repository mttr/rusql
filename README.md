# rusql

A naive, SQL based RDBMS written in Rust.

My current intentions for this project are to expand upon my pathetically
lacking knowledge of SQL, while having a bit of fun with Rust.

Based (very loosely) on SQLite, using SQLite's [understanding of SQL](https://www.sqlite.org/lang.html) as a sort of spec.

Uses [rust-peg](https://github.com/kevinmehall/rust-peg) to generate the parser.

## Example

Ripped straight out of `examples/example.rs`:

``` rust
extern crate rusql;

use rusql::{rusql_exec, Rusql, RowFormat};

fn main() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Foo VALUES(1, \"Bar1\"); \
                   INSERT INTO Foo VALUES(2, \"Bar2\"); \
                   CREATE TABLE Yarp(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Yarp VALUES(1, \"Yarp1\"); \
                   INSERT INTO Yarp VALUES(2, \"Yarp2\"); \
                   SELECT * FROM Foo, Yarp;";

    rusql_exec(&mut db, sql_str, |row, _| {
        println!("{}", RowFormat(row));
    });
}
```

``` sh
$ ./target/examples/example 
1 | Bar1 | 1 | Yarp1 |
1 | Bar1 | 2 | Yarp2 |
2 | Bar2 | 1 | Yarp1 |
2 | Bar2 | 2 | Yarp2 |
```
