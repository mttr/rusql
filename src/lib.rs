#![feature(globs)]

use std::collections::TreeMap;

pub mod parser {
    pub mod ast;
    pub mod parser;
}

pub struct Rusql<'a> {
    pub map: TreeMap<String, Table<'a>>,
}

pub type TableEntry = Vec<parser::ast::LiteralValue>;

pub struct Table<'a> {
    pub columns: Vec<parser::ast::ColumnDef>,
    pub entries: Vec<TableEntry>,
}

impl<'a> Table<'a> {
    pub fn get_column_def_by_name(&'a self, name: String) -> Option<&'a parser::ast::ColumnDef> {
        for column_def in self.columns.iter().filter(|&cols| cols.name == name) {
            return Some(column_def);
        }
        None
    }

    pub fn get_column_index(&'a self, name: String) -> Option<uint> {
        for (i, _) in self.columns.iter().filter(|&cols| cols.name == name).enumerate() {
            return Some(i);
        }
        None
    }

    pub fn has_entry(&'a self, pk: int) -> bool {
        let index = self.get_column_index("Id".to_string()).unwrap();

        for entry in self.entries.iter() {
            match entry[index] {
                parser::ast::LiteralValue::Integer(n) if n == pk => return true,
                _ => continue,
            }
        }

        false
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
                entries: Vec::new(),
            });
        }
        RusqlStatement::Insert(insert_def) => {
            match db.map.get_mut(insert_def.table_name.as_slice()) {
                Some(table) => {
                    let ref mut entries = table.entries;
                    entries.push(insert_def.column_data);
                }
                None => {},
            }
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

    fn init_db_and_insert_into_table<'a>() -> Rusql<'a> {
        let mut db = init_db_with_table();
        let sql_strs = vec![
            "INSERT INTO Foo VALUES(1, \"Bar1\");",
            "INSERT INTO Foo VALUES(2, \"Bar2\");",
            "INSERT INTO Foo VALUES(3, \"Bar3\");",
            "INSERT INTO Foo VALUES(4, \"Bar4\");",
        ];

        for sql_str in sql_strs.iter() {
            rusql_exec(&mut db, sql_str.to_string());
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
}
