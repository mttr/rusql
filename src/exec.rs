use table::{TableEntry, TableHeader, Table, get_column};
use parser::definitions::{ResultColumn, RusqlStatement, TableDef, InsertDef, SelectDef};
use parser::definitions::{DropTableDef, AlterTableDef, AlterTable, Expression, BinaryOperator};
use parser::definitions::{LiteralValue};
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
        AlterTable::AddColumn(ref column_def) => {
            let table = db.map.get_mut(alter_table_def.name.as_slice()).unwrap();
            table.header.push(column_def.clone());

            for entry in table.entries.iter_mut() {
                entry.push(LiteralValue::Null);
            }
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

#[deriving(PartialEq)]
enum ExpressionResult {
    Boolean(bool),
    Value(LiteralValue),
    //Null,
}

fn eval_boolean_expression(expr: &Expression, entry: &TableEntry, head: &TableHeader) -> bool {
    match eval_expr(expr, entry, head) {
        ExpressionResult::Boolean(b) => b,
        _ => false,
    }
}

fn eval_binary_operator(operator: BinaryOperator,
                        exp1: &Expression,
                        exp2: &Expression,
                        entry: &TableEntry,
                        head: &TableHeader) -> ExpressionResult {
    match operator {
        BinaryOperator::Equals => {
            ExpressionResult::Boolean(eval_expr(exp1, entry, head) == eval_expr(exp2, entry, head))
        }
    }
}

fn eval_expr(expr: &Expression, entry: &TableEntry, head: &TableHeader) -> ExpressionResult {
    match expr {
        &Expression::LiteralValue(ref value) => ExpressionResult::Value(value.clone()),
        &Expression::ColumnName(ref name) => ExpressionResult::Value(get_column(name, entry, head)),
        &Expression::BinaryOperator((b, ref exp1, ref exp2)) => eval_binary_operator(b, &**exp1,
                                                                                     &**exp2,
                                                                                     entry, head),
    }
}

fn select(db: &mut Rusql, select_def: &SelectDef, callback: |&TableEntry, &TableHeader|) {
    match select_def.result_column {
        ResultColumn::Asterisk => {
            for name in select_def.table_or_subquery.iter() {
                let table = db.map.get(name.as_slice()).unwrap();

                for entry in table.entries.iter() {
                    match select_def.where_expr {
                        Some(ref expr) => {
                            if !eval_boolean_expression(expr, entry, &table.header) {
                                continue;
                            }
                        }
                        None => {}
                    }
                    callback(entry, &table.header);
                }
            }
        }
    }
}
