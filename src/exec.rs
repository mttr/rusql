use table::{TableEntry, Table};
use parser::ast::{ColumnDef, ResultColumn, RusqlStatement, TableDef, InsertDef, SelectDef};
use parser::parser::rusql_stmt;
use rusql::Rusql;

pub fn rusql_exec(db: &mut Rusql, sql_str: String, callback: |&TableEntry, &Vec<ColumnDef>|) {
    let stmt = rusql_stmt(sql_str.as_slice()).unwrap();
    match stmt {
        RusqlStatement::CreateTable(table_def) => create_table(db, table_def),
        RusqlStatement::Insert(insert_def) => insert(db, insert_def),
        RusqlStatement::Select(select_def) => select(db, select_def, callback),
    }
}

fn create_table(db: &mut Rusql, table_def: TableDef) {
    db.map.insert(table_def.table_name, Table {
        columns: table_def.columns,
        entries: Vec::new(),
    });
}

fn insert(db: &mut Rusql, insert_def: InsertDef) {
    match db.map.get_mut(insert_def.table_name.as_slice()) {
        Some(table) => {
            let ref mut entries = table.entries;
            entries.push(insert_def.column_data);
        }
        None => {},
    }
}

fn select(db: &mut Rusql, select_def: SelectDef, callback: |&TableEntry, &Vec<ColumnDef>|) {
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
