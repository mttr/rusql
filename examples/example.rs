extern crate rusql;

use rusql::{rusql_exec, Rusql, TableEntry, TableHeader};

fn main() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Foo VALUES(1, \"Bar1\"); \
                   INSERT INTO Foo VALUES(2, \"Bar2\"); \
                   CREATE TABLE Yarp(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Yarp VALUES(1, \"Yarp1\"); \
                   INSERT INTO Yarp VALUES(2, \"Yarp2\"); \
                   SELECT * FROM Foo, Yarp;";

    rusql_exec(&mut db, sql_str.to_string(), |entry: &TableEntry, header: &TableHeader| {
        for (column, def) in entry.iter().zip(header.iter()) {
            println!("{}: {}", def.name, column);
        }
    });
}
