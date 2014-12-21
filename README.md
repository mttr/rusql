# rusql

A naive RDBMS written in Rust.

My current intentions for this project is to expand upon my pathetically
lacking knowledge of SQL and databases in general, while having a bit of fun
with Rust.

## Example

Ripped straight out of `examples/example.rs`:

``` rust
extern crate rusql;

use rusql::{rusql_exec, Rusql, TableEntry, TableHeader};

fn main() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Foo VALUES(1, \"Bar1\"); \
                   INSERT INTO Foo VALUES(2, \"Bar2\"); \
                   INSERT INTO Foo VALUES(3, \"Bar3\"); \
                   INSERT INTO Foo VALUES(4, \"Bar4\"); \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str.to_string(), |entry: &TableEntry, header: &TableHeader| {
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
Id: Integer(3)
Name: Text(Bar3)
Id: Integer(4)
Name: Text(Bar4)
```
