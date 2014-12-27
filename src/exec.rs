use table::{TableRow, TableHeader, Table, get_column};
use definitions::{ResultColumn, RusqlStatement, InsertDef, SelectDef};
use definitions::{AlterTableDef, AlterTable, Expression, BinaryOperator};
use definitions::{LiteralValue, DeleteDef, InsertDataSource, UpdateDef};
use rusql::Rusql;

peg_file! parser("sql.rustpeg");

pub fn rusql_exec(db: &mut Rusql, sql_str: &str, callback: |&TableRow, &TableHeader|) {
    match parser::rusql_parse(sql_str) {
        Ok(res) => {
            for stmt in res.iter() {
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
        Err(e) => println!("syntax error: {}", e),
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
        table.delete_where(|row| ExpressionEvaluator::new(row, &header, None).eval_bool(expr));
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
            let mut new_entries: Vec<TableRow> = Vec::new();

            // FIXME make sure we're putting in valid entries for this table...
            // FIXME Each row gets cloned twice...

            select(db, select_def, |row, _| {
                new_entries.push(row.clone());
            });
            let mut table = db.get_mut_table(&insert_def.table_name);

            for row in new_entries.into_iter() {
                table.push_row(row);
            }
        }
        _ => {}
    }
}

fn update(db: &mut Rusql, update_def: &UpdateDef) {
    let mut table = db.get_mut_table(&update_def.name);

    for (_, row) in table.data.iter_mut() {
        if let Some(ref expr) = update_def.where_expr {
            if !ExpressionEvaluator::new(row, &table.header, None).eval_bool(expr) {
                continue;
            }
        }

        for &(ref name, ref expr) in update_def.set.iter() {
            let x = table.header.iter().position(|ref cols| &cols.name == name).unwrap();

            row[x] = expr_to_literal(expr);
        }
    }
}

#[deriving(PartialEq)]
enum ExpressionResult {
    Boolean(bool),
    Value(LiteralValue),
    Null,
}

struct ExpressionEvaluator<'a, 'b> {
    // FIXME wtf am I doing?!?!?!
    row: &'a TableRow,
    head: &'a TableHeader,
    tables: Option<Vec<&'b Table>>,
}

impl<'a, 'b> ExpressionEvaluator<'a, 'b> {
    pub fn new(row: &'a TableRow, head: &'a TableHeader, tables: Option<Vec<&'b Table>>) -> ExpressionEvaluator<'a, 'b> {
        ExpressionEvaluator {
            row: row,
            head: head,
            tables: tables,
        }
    }

    fn eval_expr(&'a self, expr: &Expression) -> ExpressionResult {
        match expr {
            &Expression::LiteralValue(ref value) => ExpressionResult::Value(value.clone()),
            &Expression::TableName(_) => self.eval_column_name(expr, None, None),
            &Expression::ColumnName(ref name) => ExpressionResult::Value(get_column(name, self.row, self.head, None)),
            &Expression::BinaryOperator((b, ref exp1, ref exp2)) => self.eval_binary_operator(b, &**exp1,
                                                                                              &**exp2),
        }
    }

    pub fn eval_bool(&'a self, expr: &Expression) -> bool {
        match self.eval_expr(expr) {
            ExpressionResult::Boolean(b) => b,
            _ => false,
        }
    }

    fn eval_binary_operator(&'a self,
                            operator: BinaryOperator,
                            exp1: &Expression,
                            exp2: &Expression) -> ExpressionResult {
        match operator {
            BinaryOperator::Equals => {
                ExpressionResult::Boolean(self.eval_expr(exp1) == self.eval_expr(exp2))
            }
        }
    }

    fn eval_column_name(&'a self, expr: &Expression, table: Option<&Table>, offset: Option<uint>) -> ExpressionResult {
        match expr {
            &Expression::TableName((ref name, ref expr)) => {
                let mut table_opt: Option<&Table> = None;
                let mut offset = 0u;

                for table in self.tables.clone().unwrap().into_iter() {
                    if &table.name == name {
                        table_opt = Some(table);
                        break;
                    }
                    offset = offset + table.header.len();
                }
                if !table_opt.is_some() {
                    return ExpressionResult::Null;
                }

                self.eval_column_name(&**expr, table, Some(offset))
            }
            &Expression::ColumnName(ref name) => ExpressionResult::Value(get_column(name, self.row, self.head, offset)),
            _ => ExpressionResult::Boolean(false),
        }
    }
}

fn expr_to_literal(expr: &Expression) -> LiteralValue {
    match expr {
        &Expression::LiteralValue(ref literal_value) => literal_value.clone(),
        _ => LiteralValue::Null,
    }
}

fn product(tables: Vec<&Table>, result_table: &mut Table, new_row_opt: Option<TableRow>) {
    let mut remaining = tables.clone();
    if let Some(table) = remaining.remove(0) {
        for row in table.data.values() {
            let mut new_row: TableRow = if let Some(ref new_row) = new_row_opt {
                new_row.clone()
            } else {
                Vec::new()
            };

            new_row.push_all(&*row.clone());

            product(remaining.clone(), result_table, Some(new_row));
        }
    } else {
        if let Some(new_row) = new_row_opt {
            result_table.push_row(new_row);
        }
    }
}

fn select(db: &mut Rusql, select_def: &SelectDef, callback: |&TableRow, &TableHeader|) {
    let mut input_tables: Vec<&Table> = Vec::new();
    let mut result_header: TableHeader = Vec::new();

    for name in select_def.table_or_subquery.iter() {
        let table = db.get_table(name);
        input_tables.push(table);

        match select_def.result_column {
            ResultColumn::Asterisk => result_header.push_all(&*table.header.clone()),
        }
    }

    let mut result_table = Table::new_result_table(result_header);

    product(input_tables.clone(), &mut result_table, None);

    // FIXME would it be better if we did this as each row is generated?
    for row in result_table.data.values() {
        if let Some(ref expr) = select_def.where_expr {
            if !ExpressionEvaluator::new(row, &result_table.header, Some(input_tables.clone())).eval_bool(expr) {
                continue;
            }
        }
        callback(row, &result_table.header);
    }
}
