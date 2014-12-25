use table::{TableEntry, TableHeader, get_column};
use definitions::{ResultColumn, RusqlStatement, InsertDef, SelectDef};
use definitions::{AlterTableDef, AlterTable, Expression, BinaryOperator};
use definitions::{LiteralValue, DeleteDef, InsertDataSource, UpdateDef};
use rusql::Rusql;

peg_file! parser("sql.rustpeg");

pub fn rusql_exec(db: &mut Rusql, sql_str: String, callback: |&TableEntry, &TableHeader|) {
    for stmt in parser::rusql_parse(sql_str.as_slice()).unwrap().iter() {
        match stmt {
            &RusqlStatement::AlterTable(ref alter_table_def) => alter_table(db, alter_table_def),
            &RusqlStatement::CreateTable(ref table_def) => db.create_table(table_def),
            &RusqlStatement::Delete(ref delete_def) => delete(db, delete_def),
            &RusqlStatement::DropTable(ref drop_table_def) => db.drop_table(&drop_table_def.name),
            &RusqlStatement::Insert(ref insert_def) => insert(db, insert_def),
            &RusqlStatement::Select(ref select_def) => select(db, select_def, |a, b| callback(a, b)),
            &RusqlStatement::Update(ref update_def) => update(db, update_def),
        }
    }
}

fn alter_table(db: &mut Rusql, alter_table_def: &AlterTableDef) {
    match alter_table_def.mode {
        AlterTable::RenameTo(ref new_name) => db.rename_table(&alter_table_def.name, new_name),
        AlterTable::AddColumn(ref column_def) => db.get_mut_table(&alter_table_def.name)
                                                   .add_column(column_def),
    }
}

fn delete(db: &mut Rusql, delete_def: &DeleteDef) {
    let table = db.get_mut_table(&delete_def.name);

    if let Some(ref expr) = delete_def.where_expr {
        // FIXME just making the borrow checker happy...
        let header = table.header.clone();
        table.delete_where(|entry| eval_boolean_expression(expr, entry, &header));
    } else {
        table.clear();
    }
}

fn insert(db: &mut Rusql, insert_def: &InsertDef) {
    match insert_def.data_source {
        InsertDataSource::Values(ref column_data) => {
            let mut table = db.get_mut_table(&insert_def.table_name);
            table.insert(column_data, &insert_def.column_names);
        }
        InsertDataSource::Select(ref select_def) => {
            let mut new_entries: Vec<TableEntry> = Vec::new();

            // FIXME make sure we're putting in valid entries for this table...
            // FIXME Each entry gets cloned twice...

            select(db, select_def, |entry, _| {
                new_entries.push(entry.clone());
            });
            let mut table = db.get_mut_table(&insert_def.table_name);

            for entry in new_entries.into_iter() {
                table.push_entry(entry);
            }
        }
        _ => {}
    }
}

fn update(db: &mut Rusql, update_def: &UpdateDef) {
    let mut table = db.get_mut_table(&update_def.name);

    for (_, entry) in table.data.iter_mut() {
        if let Some(ref expr) = update_def.where_expr {
            if !eval_boolean_expression(expr, entry, &table.header) {
                continue;
            }
        }

        for &(ref name, ref expr) in update_def.set.iter() {
            let x = table.header.iter().position(|ref cols| &cols.name == name).unwrap();

            entry[x] = expr_to_literal(expr);
        }
    }
}

#[deriving(PartialEq)]
enum ExpressionResult {
    Boolean(bool),
    Value(LiteralValue),
    //Null,
}

fn expr_to_literal(expr: &Expression) -> LiteralValue {
    match expr {
        &Expression::LiteralValue(ref literal_value) => literal_value.clone(),
        _ => LiteralValue::Null,
    }
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
                let table = db.get_table(name);

                for entry in table.data.values() {
                    if let Some(ref expr) = select_def.where_expr {
                        if !eval_boolean_expression(expr, entry, &table.header) {
                            continue;
                        }
                    }
                    callback(entry, &table.header);
                }
            }
        }
    }
}
