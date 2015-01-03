use definitions::{Expression, LiteralValue, BinaryOperator, UnaryOperator, ColumnDef};
use table::{Table, TableRow, TableHeader, get_column};

use std::cell::Cell;

#[derive(PartialEq, Clone)]
pub enum ExpressionResult {
    Value(LiteralValue),
    ColumnDef(ColumnDef),
    Null,
}

impl ExpressionResult {
    pub fn neg(&self) -> ExpressionResult {
        match self {
            &ExpressionResult::Value(ref v) => ExpressionResult::Value(v.neg()),
            _ => self.clone(),
        }
    }
}

pub struct ExpressionEvaluator<'a, 'b> {
    // FIXME wtf am I doing?!?!?!
    row: &'a TableRow,
    head: &'a TableHeader,
    tables: Option<Vec<&'b Table>>,
    get_column_def: bool,
    as_column_alias: bool,
    order_pass: Cell<bool>,
}

impl<'a, 'b> ExpressionEvaluator<'a, 'b> {
    pub fn new(row: &'a TableRow, head: &'a TableHeader) -> ExpressionEvaluator<'a, 'b> {
        ExpressionEvaluator {
            row: row,
            head: head,
            tables: None,
            get_column_def: false,
            as_column_alias: false,
            order_pass: Cell::new(false),
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

    pub fn as_column_alias(&'a mut self) -> &mut ExpressionEvaluator<'a, 'b> {
        self.as_column_alias = true;
        self
    }

    pub fn order_of_operations(&'a self, expr: &Expression) -> Expression {
        let (b1, left1, right1) = expr.unwrap_binary_operator();
        let (b2, left2, right2) = right1.unwrap_binary_operator();

        if b1.ord_val() < b2.ord_val() {
            let right2 = self.order_of_operations(&right2);

            let new_expr_child = Expression::BinaryOperator((b1, box left2, box left1));
            let new_expr_parent = Expression::BinaryOperator((b2, box right2, box new_expr_child));

            return new_expr_parent;
        } else if b1.ord_val() == b2.ord_val() && b1 > b2{
            let right2 = self.order_of_operations(&right2);

            let new_expr_child = Expression::BinaryOperator((b1, box left1, box left2));
            let new_expr_parent = Expression::BinaryOperator((b2, box right2, box new_expr_child));

            return new_expr_parent;
        } else if b2 != BinaryOperator::Null {
            let right2 = self.order_of_operations(&right2);

            let new_expr_child = Expression::BinaryOperator((b2, box left2, box right2));
            let new_expr_parent = Expression::BinaryOperator((b1, box left1, box new_expr_child));

            return new_expr_parent;
        } else {
            return expr.clone();
        }
    }

    pub fn eval_expr(&'a self, expr: &Expression) -> ExpressionResult {
        match expr {
            &Expression::LiteralValue(ref value) => ExpressionResult::Value(value.clone()),
            &Expression::TableName(..) | &Expression::ColumnName(..) => self.eval_column_name(expr, None, None),
            &Expression::BinaryOperator((b, ref expr1, ref expr2)) =>  {
                if !self.order_pass.get() {
                    debug!("order_of_op before: {}", expr);
                    let expr = self.order_of_operations(expr);
                    debug!("order_of_op after: {}", expr);
                    self.order_pass.set(true);
                    self.eval_expr(&expr)
                } else {
                    self.eval_binary_operator(b, &**expr1, &**expr2)
                }
            }
            &Expression::UnaryOperator((u, ref exp)) => self.eval_unary_operator(u, &**exp),
            _ => ExpressionResult::Null,
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
                            expr1: &Expression,
                            expr2: &Expression) -> ExpressionResult {
        match operator {
            BinaryOperator::Less => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left.lt(&right))
            }
            BinaryOperator::LessEq => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left.le(&right))
            }
            BinaryOperator::Greater => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left.gt(&right))
            }
            BinaryOperator::GreaterEq => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left.ge(&right))
            }
            BinaryOperator::LShift => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left << right)
            }
            BinaryOperator::RShift => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left >> right)
            }
            BinaryOperator::BitAnd => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left & right)
            }
            BinaryOperator::BitOr => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left | right)
            }
            BinaryOperator::Equals => {
                ExpressionResult::Value(LiteralValue::Boolean(self.eval_expr(expr1) == self.eval_expr(expr2)))
            }
            BinaryOperator::NotEquals => {
                ExpressionResult::Value(LiteralValue::Boolean(self.eval_expr(expr1) != self.eval_expr(expr2)))
            }
            BinaryOperator::And => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(LiteralValue::Boolean(left.to_bool() && right.to_bool()))
            }
            BinaryOperator::Or => {
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(LiteralValue::Boolean(left.to_bool() || right.to_bool()))
            }
            BinaryOperator::Plus => {
                debug!("{} + {}", expr1, expr2);
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left + right)
            }
            BinaryOperator::Minus => {
                debug!("{} - {}", expr1, expr2);
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(&self.neg(expr2)));
                ExpressionResult::Value(left + right)
            }
            BinaryOperator::Mult => {
                debug!("{} * {}", expr1, expr2);
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left * right)
            }
            BinaryOperator::Divide => {
                debug!("{} / {}", expr1, expr2);
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left / right)
            }
            BinaryOperator::Modulo => {
                debug!("{} % {}", expr1, expr2);
                let left = result_to_literal(self.eval_expr(expr1));
                let right = result_to_literal(self.eval_expr(expr2));
                ExpressionResult::Value(left % right)
            }
            BinaryOperator::Null => ExpressionResult::Null,
        }
    }

    fn eval_unary_operator(&'a self, operator: UnaryOperator, expr: &Expression) -> ExpressionResult {
        debug!("{}", expr);
        match operator {
            UnaryOperator::Plus => self.eval_expr(expr),
            UnaryOperator::Minus => self.eval_expr(expr).neg(),
            UnaryOperator::Not => {
                let lit = result_to_literal(self.eval_expr(expr));
                ExpressionResult::Value(LiteralValue::Boolean(!lit.to_bool()))
            }
            UnaryOperator::BitNeg => {
                let val = result_to_literal(self.eval_expr(expr));
                ExpressionResult::Value(LiteralValue::Integer(!val.to_int()))
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
            if self.as_column_alias {
                return ExpressionResult::Value(LiteralValue::Integer(
                        // FIXME here I go with those blind unwraps again...
                        self.head.iter().position(|ref cols| &cols.name == name).unwrap() as int));
            } else {
                return ExpressionResult::Value(get_column(name, self.row, self.head, offset));
            }
        }
        ExpressionResult::Null
    }

    fn neg(&'a self, expr: &Expression) -> Expression {
        match expr {
            &Expression::LiteralValue(ref lit) => {
                match lit {
                    &LiteralValue::Integer(i) => Expression::LiteralValue(LiteralValue::Integer(-i)),
                    _ => expr.clone()
                }
            }
            &Expression::BinaryOperator((b, ref expr1, ref expr2)) => Expression::BinaryOperator((b.neg(), box self.neg(&**expr1), box self.neg(&**expr2))),
            &Expression::UnaryOperator((u, ref expr)) => Expression::UnaryOperator((u.neg(), expr.clone())),
            _ => expr.clone()
        }
    }
}

pub fn result_to_literal(result: ExpressionResult) -> LiteralValue {
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
