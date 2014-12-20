#![feature(globs)]

use std::collections::TreeMap;

pub mod parser {
    pub mod ast;
    pub mod parser;
}

pub mod table;

pub struct Rusql<'a> {
    pub map: TreeMap<String, table::Table<'a>>,
}


impl<'a> Rusql<'a> {
    pub fn new() -> Rusql<'a> {
        return Rusql {
            map: TreeMap::new(),
        };
    }
}

pub fn rusql_exec(db: &mut Rusql, sql_str: String, callback: |&table::TableEntry, &Vec<parser::ast::ColumnDef>|) {
    use parser::ast::*;
    let stmt = parser::parser::rusql_stmt(sql_str.as_slice()).unwrap();
    match stmt {
        RusqlStatement::CreateTable(table_def) => {
            db.map.insert(table_def.table_name, table::Table {
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
        RusqlStatement::Select(select_def) => {
            match select_def.result_column {
                parser::ast::ResultColumn::Asterisk => {
                    for name in select_def.table_or_subquery.iter() {
                        let table = db.map.get(name.as_slice()).unwrap();

                        for entry in table.entries.iter() {
                            callback(entry, &table.columns);
                        }
                    }
                }
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
}
