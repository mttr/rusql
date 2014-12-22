extern crate rusql;

use rusql::{rusql_exec, Rusql, LiteralValue};

fn init_db_with_table<'a>() -> Rusql<'a> {
    let mut db = rusql::Rusql::new();
    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT);".to_string();
    rusql_exec(&mut db, sql_str, |_,_| {});

    db
}

fn init_db_and_insert_into_table<'a>() -> Rusql<'a> {
    let mut db = init_db_with_table();
    let sql_strs = vec![
        "INSERT INTO Foo VALUES(1, \"Bar1\");",
        "INSERT INTO Foo VALUES(2, \"Bar2\");",
        "INSERT INTO Foo VALUES(3, \"Bar3\");",
        "INSERT INTO Foo VALUES(4, \"Bar4\");",
    ];

    for sql_str in sql_strs.iter() {
        rusql_exec(&mut db, sql_str.to_string(), |_,_| {});
    }

    db
}

#[test]
fn test_create_table() {
    let db = init_db_with_table();
    assert!(db.map.get("Foo".as_slice()).is_some());
}

#[test]
fn test_col_names() {
    let db = init_db_with_table();
    let table = db.map.get("Foo".as_slice()).unwrap();
    assert!(table.get_column_def_by_name("Id".to_string()).is_some());
    assert!(table.get_column_def_by_name("Name".to_string()).is_some());
}

#[test]
fn test_has_entries() {
    let db = init_db_and_insert_into_table();
    let table = db.map.get("Foo".as_slice()).unwrap();

    assert!(table.has_entry(1));
    assert!(table.has_entry(2));
    assert!(table.has_entry(3));
    assert!(table.has_entry(4));
}

#[test]
fn test_drop_table() {
    let mut db = init_db_with_table();
    assert!(db.map.contains_key("Foo".as_slice()));
    rusql_exec(&mut db, "DROP TABLE Foo;".to_string(), |_,_| {});
    assert!(!db.map.contains_key("Foo".as_slice()));
}

#[test]
fn test_alter_table_rename() {
    let mut db = init_db_with_table();
    assert!(db.map.contains_key("Foo".as_slice()));
    rusql_exec(&mut db, "ALTER TABLE Foo RENAME TO Bar;".to_string(), |_,_| {});
    assert!(!db.map.contains_key("Foo".as_slice()));
    assert!(db.map.contains_key("Bar".as_slice()));
}

#[test]
fn test_select_with() {
    let mut db = init_db_and_insert_into_table();
    let mut called_once = false;

    rusql_exec(&mut db, "SELECT * FROM Foo WHERE Id=2;".to_string(), |entry, _| {
        assert!(entry[0] == LiteralValue::Integer(2));
        called_once = true;
    });

    assert!(called_once);
}

#[test]
fn test_alter_table_add_to() {
    let mut db = init_db_and_insert_into_table();

    rusql_exec(&mut db, "ALTER TABLE Foo ADD COLUMN Hodor TEXT;".to_string(), |_,_| {});
    rusql_exec(&mut db, "ALTER TABLE Foo ADD Qux TEXT;".to_string(), |_,_| {});

    let table = db.map.get("Foo".as_slice()).unwrap();
    assert!(table.get_column_def_by_name("Hodor".to_string()).is_some());
    assert!(table.get_column_def_by_name("Qux".to_string()).is_some());
    table.assert_size();
}

#[test]
fn test_insert_into_with_specified_columns() {
    let mut db = init_db_with_table();
    let mut called_once = false;
    let comparison = vec![LiteralValue::Integer(3), LiteralValue::Null];

    rusql_exec(&mut db, "INSERT INTO Foo(Id) VALUES(3);".to_string(), |_,_| {});
    rusql_exec(&mut db, "SELECT * FROM Foo WHERE Id=3;".to_string(), |entry, _| {
        assert!(entry == &comparison);
        called_once = true;
    });

    assert!(called_once);
}
