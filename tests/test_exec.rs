extern crate rusql;

use rusql::{rusql_exec, Rusql, LiteralValue};

fn init_db_with_table() -> Rusql {
    let mut db = rusql::Rusql::new();
    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT);";
    rusql_exec(&mut db, sql_str, |_,_| {});

    db
}

fn init_db_and_insert_into_table() -> Rusql {
    let mut db = init_db_with_table();
    let sql_strs = vec![
        "INSERT INTO Foo VALUES(1, \"Bar1\");",
        "INSERT INTO Foo VALUES(2, \"Bar2\");",
        "INSERT INTO Foo VALUES(3, \"Bar3\");",
        "INSERT INTO Foo VALUES(4, \"Bar4\");",
    ];

    for sql_str in sql_strs.iter() {
        rusql_exec(&mut db, *sql_str, |_,_| {});
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
    assert!(table.get_column_def_by_name(&"Id".to_string()).is_some());
    assert!(table.get_column_def_by_name(&"Name".to_string()).is_some());
}

#[test]
fn test_has_entries() {
    let db = init_db_and_insert_into_table();
    let table = db.map.get("Foo".as_slice()).unwrap();

    assert!(table.has_row(1));
    assert!(table.has_row(2));
    assert!(table.has_row(3));
    assert!(table.has_row(4));
}

#[test]
fn test_drop_table() {
    let mut db = init_db_with_table();
    assert!(db.map.contains_key("Foo".as_slice()));
    rusql_exec(&mut db, "DROP TABLE Foo;", |_,_| {});
    assert!(!db.map.contains_key("Foo".as_slice()));
}

#[test]
fn test_alter_table_rename() {
    let mut db = init_db_with_table();
    assert!(db.map.contains_key("Foo".as_slice()));
    rusql_exec(&mut db, "ALTER TABLE Foo RENAME TO Bar;", |_,_| {});
    assert!(!db.map.contains_key("Foo".as_slice()));
    assert!(db.map.contains_key("Bar".as_slice()));
}

#[test]
fn test_select_with() {
    let mut db = init_db_and_insert_into_table();
    let mut called_once = false;

    rusql_exec(&mut db, "SELECT * FROM Foo WHERE Id=2;", |row, _| {
        assert!(row[0] == LiteralValue::Integer(2));
        called_once = true;
    });

    assert!(called_once);
}

#[test]
fn test_alter_table_add_to() {
    let mut db = init_db_and_insert_into_table();

    rusql_exec(&mut db, "ALTER TABLE Foo ADD COLUMN Hodor TEXT;", |_,_| {});
    rusql_exec(&mut db, "ALTER TABLE Foo ADD Qux TEXT;", |_,_| {});

    let table = db.map.get("Foo".as_slice()).unwrap();
    assert!(table.get_column_def_by_name(&"Hodor".to_string()).is_some());
    assert!(table.get_column_def_by_name(&"Qux".to_string()).is_some());
    table.assert_size();
}

#[test]
fn test_insert_into_with_specified_columns() {
    let mut db = init_db_with_table();
    let mut called_once = false;
    let comparison = vec![LiteralValue::Integer(3), LiteralValue::Null];

    rusql_exec(&mut db, "INSERT INTO Foo(Id) VALUES(3);", |_,_| {});
    rusql_exec(&mut db, "SELECT * FROM Foo WHERE Id=3;", |row, _| {
        assert!(row == &comparison);
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

    rusql_exec(&mut db, sql_str, |row, _| {
        results.push(row[0].clone());
    });

    assert_eq!(results, expected);
}

#[test]
fn test_delete_all() {
    let mut db = init_db_and_insert_into_table();

    rusql_exec(&mut db, "DELETE FROM Foo;", |_,_| {});

    let table = db.get_table(&"Foo".to_string());
    assert!(table.data.len() == 0);
}

#[test]
fn test_delete_with() {
    let mut db = init_db_and_insert_into_table();
    let expected = vec![1, 2, 4];
    let mut results: Vec<int> = Vec::new();

    let sql_str = "DELETE FROM Foo WHERE Id=3; \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str, |row, _| {
        match row[0] {
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

    rusql_exec(&mut db, sql_str, |_,_| {});

    let foo = db.get_table(&"Foo".to_string());
    let foo2 = db.get_table(&"Foo2".to_string());

    assert!(foo.data == foo2.data);
}

#[test]
fn test_update() {
    let mut db = init_db_and_insert_into_table();
    let sql_str = "UPDATE Foo SET Name=\"Qux\"; \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str, |row, _| {
        assert!(row[1] == LiteralValue::Text("Qux".to_string()));
    });
}

#[test]
fn test_update_where() {
    let mut db = init_db_and_insert_into_table();
    let sql_str = "UPDATE Foo SET Name=\"Qux\" WHERE Id=3; \
                   SELECT * FROM Foo WHERE Id=3;";
    let expected = vec![LiteralValue::Text("Qux".to_string())];
    let mut results: Vec<LiteralValue> = Vec::new();

    rusql_exec(&mut db, sql_str, |row, _| {
        results.push(row[1].clone());
    });

    assert!(results == expected);
}

#[test]
fn test_select_multiple_tables() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE a(Num INTEGER); \
                   CREATE TABLE b(Num INTEGER); \
                   CREATE TABLE c(Num INTEGER); \
                   INSERT INTO a VALUES(1), (2), (3); \
                   INSERT INTO b VALUES(1), (2); \
                   INSERT INTO c VALUES(1); \
                   SELECT * FROM a, b, c;";

    let expected = vec![vec![1, 1, 1],
                        vec![1, 2, 1],
                        vec![2, 1, 1],
                        vec![2, 2, 1],
                        vec![3, 1, 1],
                        vec![3, 2, 1]];
    let mut results: Vec<Vec<int>> = Vec::new();

    rusql_exec(&mut db, sql_str, |row, _| {
        let mut result_row: Vec<int> = Vec::new();
        for column in row.iter() {
            result_row.push(column.to_uint() as int);
        }
        results.push(result_row);
    });

    assert_eq!(results, expected);
}

#[test]
fn test_select_with_mutltiple_tables() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE a(Num INTEGER); \
                   CREATE TABLE b(Num INTEGER); \
                   INSERT INTO a VALUES(1), (2), (3); \
                   INSERT INTO b VALUES(1), (2); \
                   SELECT * FROM a, b WHERE a.Num=b.Num;";

    let expected = vec![vec![1, 1],
                        vec![2, 2]];
    let mut results: Vec<Vec<int>> = Vec::new();

    rusql_exec(&mut db, sql_str, |row, _| {
        let mut result_row: Vec<int> = Vec::new();
        for column in row.iter() {
            result_row.push(column.to_uint() as int);
        }
        results.push(result_row);
    });

    assert_eq!(results, expected);
}

#[test]
fn test_select_multiple_tables_with_specified_result_columns() {
    let mut db = Rusql::new();

    let sql_str = "CREATE TABLE a(Num INTEGER); \
                   CREATE TABLE b(Num INTEGER); \
                   CREATE TABLE c(Num INTEGER); \
                   INSERT INTO a VALUES(1), (2), (3); \
                   INSERT INTO b VALUES(1), (2); \
                   INSERT INTO c VALUES(1); \
                   SELECT c.Num, a.Num, b.Num FROM a, b, c;";

    let expected = vec![vec![1, 1, 1],
                        vec![1, 1, 2],
                        vec![1, 2, 1],
                        vec![1, 2, 2],
                        vec![1, 3, 1],
                        vec![1, 3, 2]];
    let mut results: Vec<Vec<int>> = Vec::new();

    rusql_exec(&mut db, sql_str, |row, _| {
        let mut result_row: Vec<int> = Vec::new();
        for column in row.iter() {
            result_row.push(column.to_uint() as int);
        }
        results.push(result_row);
    });

    assert_eq!(results, expected);
}

#[test]
fn test_select_header_length_specified_table_and_columns() {
    let mut db = init_db_and_insert_into_table();

    let results = rusql_exec(&mut db, "SELECT Foo.Id, Foo.Name FROM Foo;", |_,_| {}).unwrap();
    assert_eq!(results.header.len(), 2);
}

#[test]
fn test_select_header_length_specified_columns() {
    let mut db = init_db_and_insert_into_table();

    let results = rusql_exec(&mut db, "SELECT Id, Name FROM Foo;", |_,_| {}).unwrap();
    assert_eq!(results.header.len(), 2);
}

#[test]
fn test_select_header_length_asterisk() {
    let mut db = init_db_and_insert_into_table();

    let results = rusql_exec(&mut db, "SELECT * FROM Foo;", |_,_| {}).unwrap();
    assert_eq!(results.header.len(), 2);
}

#[test]
fn test_select_order_by_asc() {
    let mut db = Rusql::new();
    let expected = vec![1, 2, 3, 4];
    let mut results: Vec<int> = Vec::new();

    let sql_str = "CREATE TABLE a(b INTEGER); \
                   INSERT INTO a VALUES \
                       (4), \
                       (2), \
                       (1), \
                       (3); \
                   SELECT * FROM a ORDER BY b;";
    rusql_exec(&mut db, sql_str, |row,_| {
        for column in row.iter() {
            results.push(column.to_uint() as int);
        }
    });

    assert_eq!(expected, results);
}

#[test]
fn test_select_order_by_desc() {
    let mut db = Rusql::new();
    let expected = vec![4, 3, 2, 1];
    let mut results: Vec<int> = Vec::new();

    let sql_str = "CREATE TABLE a(b INTEGER); \
                   INSERT INTO a VALUES \
                       (4), \
                       (2), \
                       (1), \
                       (3); \
                   SELECT * FROM a ORDER BY b DESC;";
    rusql_exec(&mut db, sql_str, |row,_| {
        for column in row.iter() {
            results.push(column.to_uint() as int);
        }
    });

    assert_eq!(expected, results);
}

#[test]
fn test_single_quote() {
    let mut db = init_db_with_table();
    let mut results: Vec<LiteralValue> = Vec::new();
    let expected = vec![LiteralValue::Text("Bar".to_string())];
    let sql_str = "INSERT INTO Foo VALUES(1, 'Bar'); \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str, |row, _| {
        results.push(row[1].clone());
    });

    assert_eq!(expected, results);
}

#[test]
fn test_if_not_exists() {
    let mut db = init_db_with_table();
    rusql_exec(&mut db, "CREATE TABLE IF NOT EXISTS Foo(Num INTEGER PRIMARY KEY, Nickname TEXT);", |_, _| {});

    let table = db.map.get("Foo".as_slice()).unwrap();
    assert!(table.get_column_def_by_name(&"Id".to_string()).is_some());
    assert!(table.get_column_def_by_name(&"Name".to_string()).is_some());
}

#[test]
fn test_pk_auto_increment() {
    let mut db = init_db_with_table();
    let mut results: Vec<int> = Vec::new();
    let expected = vec![1, 2, 3, 4];
    let sql_str = "INSERT INTO Foo(Name) VALUES(\"Bar0\"), (\"Bar1\"), (\"Bar2\"), (\"Bar3\"); \
                   SELECT * FROM Foo;";

    rusql_exec(&mut db, sql_str, |row, _| {
        results.push(row[0].to_int());
    });

    assert_eq!(expected, results);
}
