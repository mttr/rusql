#![feature(globs)]

use std::collections::TreeMap;

pub mod parser {
    pub mod ast;
    pub mod parser;
}

pub struct Rusql {
    pub map: TreeMap<String, Table>,
}

pub struct Table {
    pub columns: Vec<parser::ast::ColumnDef>,
}

impl Rusql {
    pub fn new() -> Rusql {
        return Rusql {
            map: TreeMap::new(),
        };
    }
}

pub fn rusql_exec(db: &mut Rusql, sql_str: String) {
    use parser::ast::*;
    let stmt = parser::parser::rusql_stmt(sql_str.as_slice()).unwrap();
    match stmt {
        RusqlStatement::CreateTable(table_def) => {
            db.map.insert(table_def.table_name, Table {
                columns: table_def.columns,
            });
        }
    }
}

#[test]
fn test_create_table() {
    let mut db = Rusql::new();
    let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT);".to_string();
    rusql_exec(&mut db, sql_str);
    assert!(db.map.get("Foo".as_slice()).is_some());
}
