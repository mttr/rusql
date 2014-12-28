use definitions::{Expression, LiteralValue, BinaryOperator, ColumnDef};
use table::{Table, TableRow, TableHeader, get_column};

#[deriving(PartialEq)]
pub enum ExpressionResult {
    Boolean(bool),
    Value(LiteralValue),
    ColumnDef(ColumnDef),
    Null,
}

pub struct ExpressionEvaluator<'a, 'b> {
    // FIXME wtf am I doing?!?!?!
    row: &'a TableRow,
    head: &'a TableHeader,
    tables: Option<Vec<&'b Table>>,
    get_column_def: bool,
}

impl<'a, 'b> ExpressionEvaluator<'a, 'b> {
    pub fn new(row: &'a TableRow, head: &'a TableHeader, tables: Option<Vec<&'b Table>>, get_column_def: bool) -> ExpressionEvaluator<'a, 'b> {
        ExpressionEvaluator {
            row: row,
            head: head,
            tables: tables,
            get_column_def: get_column_def,
        }
    }

    pub fn eval_expr(&'a self, expr: &Expression) -> ExpressionResult {
        match expr {
            &Expression::LiteralValue(ref value) => ExpressionResult::Value(value.clone()),
            &Expression::TableName(_) | &Expression::ColumnName(_) => self.eval_column_name(expr, None, None),
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

                self.eval_column_name(&**expr, table_opt, Some(offset))
            }
            &Expression::ColumnName(ref name) => self.column_data_or_def(name, table, offset),
            _ => ExpressionResult::Null,
        }
    }

    fn column_data_or_def(&'a self, name: &String, table: Option<&Table>, offset: Option<uint>) -> ExpressionResult {
        if self.get_column_def {
            if let Some(table) = table {
                // We know which table to grab the def from...
                if let Some(column_def) = table.get_column_def_by_name(name) {
                    return ExpressionResult::ColumnDef(column_def.clone());
                }
            } else {
                // FIXME what if there are _other_ columns with the same name
                // further down?
                if let Some(ref tables) = self.tables {
                    for table in tables.iter() {
                        if let Some(column_def) = table.get_column_def_by_name(name) {
                            return ExpressionResult::ColumnDef(column_def.clone());
                        }
                    }
                }
            }
        } else {
            return ExpressionResult::Value(get_column(name, self.row, self.head, offset));
        }
        ExpressionResult::Null
    }
}

pub fn expr_to_literal(expr: &Expression) -> LiteralValue {
    match expr {
        &Expression::LiteralValue(ref literal_value) => literal_value.clone(),
        _ => LiteralValue::Null,
    }
}
