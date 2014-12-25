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

#[test]
fn test_insert_into_with_multiple_rows() {
    let mut db = Rusql::new();
    let expected = vec![LiteralValue::Integer(2),
                        LiteralValue::Integer(4),
                        LiteralValue::Integer(8),
                        LiteralValue::Integer(15),
                        LiteralValue::Integer(16),
                        LiteralValue::Integer(23),
                        LiteralValue::Integer(42)];
    let mut results: Vec<LiteralValue> = Vec::new();

    let sql_str = "CREATE TABLE Ints(Id INTEGER PRIMARY KEY); \
                   INSERT INTO Ints(Id) VALUES (2), (4), (8), (15), (16), (23), (42); \
                   SELECT * FROM Ints;";

    rusql_exec(&mut db, sql_str.to_string(), |entry, _| {
        results.push(entry[0].clone());
    });

    assert!(results == expected);
}

#[test]
fn test_delete_all() {
    let mut db = init_db_and_insert_into_table();

    rusql_exec(&mut db, "DELETE FROM Foo;".to_string(), |_,_| {});

    let table = db.get_table(&"Foo".to_string());
    assert!(table.entries.len() == 0);
}

#[test]
fn test_delete_with() {
    let mut db = init_db_and_insert_into_table();
    let expected = vec![1, 2, 4];
    let mut results: Vec<int> = Vec::new();

    let sql_str = "DELETE FROM Foo WHERE Id=3; \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str.to_string(), |entry, _| {
        match entry[0] {
            LiteralValue::Integer(id) => results.push(id),
            _ => {}
        }
    });

    assert!(results == expected);
}

#[test]
fn test_insert_with_select() {
    let mut db = init_db_and_insert_into_table();
    let sql_str = "CREATE TABLE Foo2(Id INTEGER PRIMARY KEY, Name TEXT); \
                   INSERT INTO Foo2 SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str.to_string(), |_,_| {});

    let foo = db.get_table(&"Foo".to_string());
    let foo2 = db.get_table(&"Foo2".to_string());

    assert!(foo.entries == foo2.entries);
}

#[test]
fn test_update() {
    let mut db = init_db_and_insert_into_table();
    let sql_str = "UPDATE Foo SET Name=\"Qux\"; \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str.to_string(), |entry, _| {
        assert!(entry[1] == LiteralValue::Text("Qux".to_string()));
    });
}

#[test]
fn test_update_where() {
    let mut db = init_db_and_insert_into_table();
    let sql_str = "UPDATE Foo SET Name=\"Qux\" WHERE Id=3; \
                   SELECT * FROM Foo WHERE Id=3;";
    let expected = vec![LiteralValue::Text("Qux".to_string())];
    let mut results: Vec<LiteralValue> = Vec::new();

    rusql_exec(&mut db, sql_str.to_string(), |entry, _| {
        results.push(entry[1].clone());
    });

    assert!(results == expected);
}
