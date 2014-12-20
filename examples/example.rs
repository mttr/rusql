extern crate rusql;

use rusql::exec::rusql_exec;
use rusql::rusql::Rusql;
use rusql::table::TableEntry;
use rusql::parser::ast::ColumnDef;

fn main() {
    let mut db = Rusql::new();

    rusql_exec(&mut db, "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT);".to_string(), |_,_| {});

    let sql_strs = vec![
        "INSERT INTO Foo VALUES(1, \"Bar1\");",
        "INSERT INTO Foo VALUES(2, \"Bar2\");",
        "INSERT INTO Foo VALUES(3, \"Bar3\");",
        "INSERT INTO Foo VALUES(4, \"Bar4\");",
    ];

    for sql_str in sql_strs.iter() {
        rusql_exec(&mut db, sql_str.to_string(), |_,_| {});
    }

    rusql_exec(&mut db, "SELECT * FROM Foo;".to_string(), |entry: &TableEntry, def: &Vec<ColumnDef>| {
        for (column, column_def) in entry.iter().zip(def.iter()) {
            println!("{}: {}", column_def.name, column);
        }
    });
}
