use table::{TableEntry, TableHeader, Table};
use parser::definitions::{ResultColumn, RusqlStatement, TableDef, InsertDef, SelectDef};
use parser::definitions::{DropTableDef, AlterTableDef, AlterTable};
use parser::parser::rusql_parse;
use rusql::Rusql;

pub fn rusql_exec(db: &mut Rusql, sql_str: String, callback: |&TableEntry, &TableHeader|) {
    for stmt in rusql_parse(sql_str.as_slice()).unwrap().iter() {
        match stmt {
            &RusqlStatement::AlterTable(ref alter_table_def) => alter_table(db, alter_table_def),
            &RusqlStatement::CreateTable(ref table_def) => create_table(db, table_def),
            &RusqlStatement::DropTable(ref drop_table_def) => drop_table(db, drop_table_def),
            &RusqlStatement::Insert(ref insert_def) => insert(db, insert_def),
            &RusqlStatement::Select(ref select_def) => select(db, select_def, |a, b| callback(a, b)),
        }
    }
}

fn alter_table(db: &mut Rusql, alter_table_def: &AlterTableDef) {
    match alter_table_def.mode {
        AlterTable::RenameTo(ref new_name) => {
            let table = db.map.remove(alter_table_def.name.as_slice()).unwrap();
            db.map.insert(new_name.clone(), table);
        }
    }
}

fn create_table(db: &mut Rusql, table_def: &TableDef) {
    db.map.insert(table_def.table_name.clone(), Table {
        header: table_def.columns.clone(),
        entries: Vec::new(),
    });
}

fn drop_table(db: &mut Rusql, drop_table_def: &DropTableDef) {
    db.map.remove(drop_table_def.name.as_slice());
}

fn insert(db: &mut Rusql, insert_def: &InsertDef) {
    match db.map.get_mut(insert_def.table_name.as_slice()) {
        Some(table) => {
            let ref mut entries = table.entries;
            entries.push(insert_def.column_data.clone());
        }
        None => {},
    }
}

fn select(db: &mut Rusql, select_def: &SelectDef, callback: |&TableEntry, &TableHeader|) {
    match select_def.result_column {
        ResultColumn::Asterisk => {
            for name in select_def.table_or_subquery.iter() {
                let table = db.map.get(name.as_slice()).unwrap();

                for entry in table.entries.iter() {
                    callback(entry, &table.header);
                }
            }
        }
    }
}
