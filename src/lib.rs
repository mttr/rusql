#![feature(globs)]

use std::collections::TreeMap;

pub mod parser {
    pub mod ast;
    pub mod parser;
}

pub struct Rusql<'a> {
    pub map: TreeMap<String, Table<'a>>,
}

pub struct Table<'a> {
    pub columns: Vec<parser::ast::ColumnDef>,
}

impl<'a> Table<'a> {
    pub fn get_column_def_by_name(&'a self, name: String) -> Option<&'a parser::ast::ColumnDef> {
        for column_def in self.columns.iter().filter(|&col| col.name == name) {
            return Some(column_def);
        }
        None
    }
}
impl<'a> Rusql<'a> {
    pub fn new() -> Rusql<'a> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init_db_with_table<'a>() -> Rusql<'a> {
        let mut db = Rusql::new();
        let sql_str = "CREATE TABLE Foo(Id INTEGER PRIMARY KEY, Name TEXT);".to_string();
        rusql_exec(&mut db, sql_str);

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
}
