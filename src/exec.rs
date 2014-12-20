use table::{TableEntry, Table};
use parser::ast::{ColumnDef, ResultColumn, RusqlStatement};
use parser::parser::rusql_stmt;
use super::Rusql;

pub fn rusql_exec(db: &mut Rusql, sql_str: String, callback: |&TableEntry, &Vec<ColumnDef>|) {
    let stmt = rusql_stmt(sql_str.as_slice()).unwrap();
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
        RusqlStatement::Select(select_def) => {
            match select_def.result_column {
                ResultColumn::Asterisk => {
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
