# rusql

A naive RDBMS written in Rust.

My current intentions for this project is to expand upon my pathetically
lacking knowledge of SQL and databases in general, while having a bit of fun
with Rust.

## Example

Ripped straight out of `examples/example.rs`:

``` rust
extern crate rusql;

use rusql::{rusql_exec, Rusql};

fn main() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Foo VALUES(1, \"Bar1\"); \
                   INSERT INTO Foo VALUES(2, \"Bar2\"); \
                   CREATE TABLE Yarp(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Yarp VALUES(1, \"Yarp1\"); \
                   INSERT INTO Yarp VALUES(2, \"Yarp2\"); \
                   SELECT * FROM Foo, Yarp;";

    rusql_exec(&mut db, sql_str.to_string(), |entry, header| {
        for (column, def) in entry.iter().zip(header.iter()) {
            println!("{}: {}", def.name, column);
        }
    });
}
```

``` sh
$ ./target/examples/example 
Id: Integer(1)
Name: Text(Bar1)
Id: Integer(2)
Name: Text(Bar2)
Id: Integer(1)
Name: Text(Yarp1)
Id: Integer(2)
Name: Text(Yarp2)
```
