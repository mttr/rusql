use definitions::{Expression, LiteralValue, BinaryOperator, UnaryOperator, ColumnDef};
use table::{Table, TableRow, TableHeader, get_column};

#[deriving(PartialEq)]
pub enum ExpressionResult {
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
    pub fn new(row: &'a TableRow, head: &'a TableHeader) -> ExpressionEvaluator<'a, 'b> {
        ExpressionEvaluator {
            row: row,
            head: head,
            tables: None,
            get_column_def: false,
        }
    }

    pub fn with_column_def(&'a mut self) -> &mut ExpressionEvaluator<'a, 'b> {
        self.get_column_def = true;
        self
    }

    pub fn with_tables(&'a mut self, tables: Vec<&'b Table>) -> &mut ExpressionEvaluator<'a, 'b> {
        self.tables = Some(tables);
        self
    }

    pub fn eval_expr(&'a self, expr: &Expression) -> ExpressionResult {
        match expr {
            &Expression::LiteralValue(ref value) => ExpressionResult::Value(value.clone()),
            &Expression::TableName(_) | &Expression::ColumnName(_) => self.eval_column_name(expr, None, None),
            &Expression::BinaryOperator((b, ref exp1, ref exp2)) => self.eval_binary_operator(b, &**exp1,
                                                                                              &**exp2),
            &Expression::UnaryOperator((u, ref exp)) => self.eval_unary_operator(u, &**exp),
        }
    }

    pub fn eval_bool(&'a self, expr: &Expression) -> bool {
        match self.eval_expr(expr) {
            ExpressionResult::Value(value) => {
                match value {
                    LiteralValue::Boolean(b) => b,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn eval_binary_operator(&'a self,
                            operator: BinaryOperator,
                            exp1: &Expression,
                            exp2: &Expression) -> ExpressionResult {
        match operator {
            BinaryOperator::Equals => {
                ExpressionResult::Value(LiteralValue::Boolean(self.eval_expr(exp1) == self.eval_expr(exp2)))
            }
            BinaryOperator::Plus => {
                debug!("{} + {}", exp1, exp2);
                let left = result_to_literal(self.eval_expr(exp1));
                let right = result_to_literal(self.eval_expr(exp2));
                ExpressionResult::Value(LiteralValue::Integer(left.to_int() + right.to_int()))
            }
            BinaryOperator::Minus => {
                debug!("{} - {}", exp1, exp2);
                let left = result_to_literal(self.eval_expr(exp1));
                let right = result_to_literal(self.eval_expr(&neg(exp2)));
                ExpressionResult::Value(LiteralValue::Integer(left.to_int() + right.to_int()))
            }
        }
    }

    fn eval_unary_operator(&'a self, operator: UnaryOperator, expr: &Expression) -> ExpressionResult {
        match operator {
            UnaryOperator::Plus => self.eval_expr(expr),
            UnaryOperator::Minus => ExpressionResult::Value(LiteralValue::Integer(-expr_to_literal(expr).to_int())),
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

fn neg(expr: &Expression) -> Expression {
    match expr {
        &Expression::LiteralValue(ref lit) => {
            match lit {
                &LiteralValue::Integer(i) => Expression::LiteralValue(LiteralValue::Integer(-i)),
                _ => expr.clone()
            }
        }
        &Expression::BinaryOperator((b, ref expr1, ref expr2)) => Expression::BinaryOperator((b.neg(), box neg(&**expr1), box neg(&**expr2))),
        &Expression::UnaryOperator((u, ref expr)) => Expression::UnaryOperator((u.neg(), expr.clone())),
        _ => expr.clone()
    }
}

fn result_to_literal(result: ExpressionResult) -> LiteralValue {
    match result {
        ExpressionResult::Value(v) => v,
        _ => LiteralValue::Null,
    }
}

pub fn expr_to_literal(expr: &Expression) -> LiteralValue {
    match expr {
        &Expression::LiteralValue(ref literal_value) => literal_value.clone(),
        _ => LiteralValue::Null,
    }
}
