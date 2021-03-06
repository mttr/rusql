use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, Shl, Shr};

pub enum RusqlStatement {
    AlterTable(AlterTableDef),
    CreateTable(TableDef),
    Delete(DeleteDef),
    DropTable(DropTableDef),
    Insert(InsertDef),
    Select(SelectDef),
    Update(UpdateDef),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    Text,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ColumnConstraint {
    PrimaryKey,
}

#[derive(Show, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(isize),
    Text(String),
    Real(f64),
    Boolean(bool),
    Null,
}

impl LiteralValue {
    pub fn to_uint(&self) -> usize {
        match self {
            &LiteralValue::Integer(i) => i as usize,
            _ => 0, // FIXME ???
        }
    }
    pub fn to_int(&self) -> isize {
        match self {
            &LiteralValue::Integer(i) => i,
            &LiteralValue::Boolean(b) => if b { 1 } else { 0 },
            _ => 0, // FIXME ???
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            &LiteralValue::Integer(i) => i != 0,
            &LiteralValue::Boolean(b) => b,
            _ => false, // FIXME ???
        }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        if self.is_int() && other.is_int() {
            let x = self.to_int();
            let y = other.to_int();

            return x.cmp(&y)
        }

        Equal
    }

    pub fn lt(&self, other: &Self) -> LiteralValue {
        match self.cmp(other) {
            Less => LiteralValue::Boolean(true),
            Equal => LiteralValue::Boolean(false),
            Greater => LiteralValue::Boolean(false),
        }
    }

    pub fn le(&self, other: &Self) -> LiteralValue {
        match self.cmp(other) {
            Less => LiteralValue::Boolean(true),
            Equal => LiteralValue::Boolean(true),
            Greater => LiteralValue::Boolean(false),
        }
    }

    pub fn gt(&self, other: &Self) -> LiteralValue {
        match self.cmp(other) {
            Less => LiteralValue::Boolean(false),
            Equal => LiteralValue::Boolean(false),
            Greater => LiteralValue::Boolean(true),
        }
    }

    pub fn ge(&self, other: &Self) -> LiteralValue {
        match self.cmp(other) {
            Less => LiteralValue::Boolean(false),
            Equal => LiteralValue::Boolean(true),
            Greater => LiteralValue::Boolean(true),
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            &LiteralValue::Integer(..) => true,
            _ => false,
        }
    }

    pub fn neg(&self) -> LiteralValue {
        match self {
            &LiteralValue::Integer(i) => LiteralValue::Integer(-i),
            _ => self.clone(),
        }
    }

    fn int_add(&self, x: isize, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x + i),
            _ => LiteralValue::Null,
        }
    }

    fn int_sub(&self, x: isize, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x - i),
            _ => LiteralValue::Null,
        }
    }

    fn int_mul(&self, x: isize, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x * i),
            _ => LiteralValue::Null,
        }
    }

    fn int_div(&self, x: isize, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x / i),
            _ => LiteralValue::Null,
        }
    }

    fn int_rem(&self, x: isize, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x % i),
            _ => LiteralValue::Null,
        }
    }
}

impl fmt::String for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LiteralValue::Integer(ref i) => write!(f, "{}", i),
            &LiteralValue::Text(ref t) => write!(f, "{}", t),
            &LiteralValue::Real(ref r) => write!(f, "{}", r),
            &LiteralValue::Boolean(ref b) => write!(f, "{}", b),
            &LiteralValue::Null => write!(f, "null"),
        }
    }
}

impl Add for LiteralValue {
    type Output = LiteralValue;
    fn add(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_add(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Sub for LiteralValue {
    type Output = LiteralValue;
    fn sub(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_sub(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Mul for LiteralValue {
    type Output = LiteralValue;
    fn mul(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_mul(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Div for LiteralValue {
    type Output = LiteralValue;
    fn div(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_div(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Rem for LiteralValue {
    type Output = LiteralValue;
    fn rem(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_rem(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl BitAnd for LiteralValue {
    type Output = LiteralValue;
    fn bitand(self, rhs: LiteralValue) -> LiteralValue {
        if self.is_int() && rhs.is_int() {
            LiteralValue::Integer(self.to_int() & rhs.to_int())
        } else {
            LiteralValue::Null
        }
    }
}

impl BitOr for LiteralValue {
    type Output = LiteralValue;
    fn bitor(self, rhs: LiteralValue) -> LiteralValue {
        if self.is_int() && rhs.is_int() {
            LiteralValue::Integer(self.to_int() | rhs.to_int())
        } else {
            LiteralValue::Null
        }
    }
}

impl Shl<LiteralValue> for LiteralValue {
    type Output = LiteralValue;
    fn shl(self, rhs: LiteralValue) -> LiteralValue {
        if self.is_int() && rhs.is_int() {
            LiteralValue::Integer(self.to_int() << rhs.to_int() as usize)
        } else {
            LiteralValue::Null
        }
    }
}

impl Shr<LiteralValue> for LiteralValue {
    type Output = LiteralValue;
    fn shr(self, rhs: LiteralValue) -> LiteralValue {
        if self.is_int() && rhs.is_int() {
            LiteralValue::Integer(self.to_int() >> rhs.to_int() as usize)
        } else {
            LiteralValue::Null
        }
    }
}



pub struct TableDef {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
    pub if_not_exists: bool,
}

pub enum ResultColumn {
    Expressions(Vec<Expression>),
    Asterisk,
}

pub struct InsertDef {
    pub table_name: String,
    pub column_names: Option<Vec<String>>,
    pub data_source: InsertDataSource,
}

pub enum InsertDataSource {
    Values(Vec<Vec<LiteralValue>>),
    Select(SelectDef),
    DefaultValues,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub column_type: Option<ColumnType>,
    pub column_constraints: Vec<ColumnConstraint>,
}

pub struct SelectDef {
    pub result_column: ResultColumn,
    pub from_clause: Option<FromClause>,
    pub where_expr: Option<Expression>,
    pub ordering_terms: Option<Vec<OrderingTerm>>,
}

pub struct DropTableDef {
    pub name: String,
}

pub enum AlterTable {
    RenameTo(String),
    AddColumn(ColumnDef),
}

pub struct AlterTableDef {
    pub name: String,
    pub mode: AlterTable,
}

#[derive(Show, Clone)]
pub enum Expression {
    LiteralValue(LiteralValue),
    TableName((String, Box<Expression>)),
    ColumnName(String),
    BinaryOperator((BinaryOperator, Box<Expression>, Box<Expression>)),
    UnaryOperator((UnaryOperator, Box<Expression>)),
    Null,
}

impl Expression {
    pub fn unwrap_binary_operator(&self) -> (BinaryOperator, Expression, Expression) {
        match self {
            &Expression::BinaryOperator((b, ref left, ref right)) => (b, *left.clone(), *right.clone()),
            _ => (BinaryOperator::Null, Expression::Null, Expression::Null),
        }
    }
}

#[derive(Copy, Show, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Null,
    Mult,
    Divide,
    Modulo,
    Plus,
    Minus,
    LShift,
    RShift,
    BitAnd,
    BitOr,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equals,
    NotEquals,
    And,
    Or,
}

impl BinaryOperator {
    pub fn neg(&self) -> BinaryOperator {
        match *self {
            BinaryOperator::Plus => BinaryOperator::Minus,
            BinaryOperator::Minus => BinaryOperator::Plus,
            _ => *self
        }
    }

    pub fn ord_val(&self) -> usize {
        match *self {
            BinaryOperator::Null => 0,
            BinaryOperator::Mult | BinaryOperator::Divide | BinaryOperator::Modulo => 2,
            BinaryOperator::Plus | BinaryOperator::Minus => 3,
            BinaryOperator::LShift | BinaryOperator::RShift
                | BinaryOperator::BitAnd | BinaryOperator::BitOr => 4,
            BinaryOperator::Less | BinaryOperator::LessEq
                | BinaryOperator::Greater | BinaryOperator::GreaterEq => 5,
            BinaryOperator::Equals | BinaryOperator::NotEquals => 6,
            BinaryOperator::And => 7,
            BinaryOperator::Or => 8,
        }
    }
}

#[derive(Copy, Show, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitNeg,
}

impl UnaryOperator {
    pub fn neg(&self) -> UnaryOperator {
        match *self {
            UnaryOperator::Plus => UnaryOperator::Minus,
            UnaryOperator::Minus => UnaryOperator::Plus,
            _ => *self,
        }
    }
}

pub struct DeleteDef {
    pub name: String,
    pub where_expr: Option<Expression>,
}

pub struct UpdateDef {
    pub name: String,
    pub set: Vec<(String, Expression)>,
    pub where_expr: Option<Expression>,
}

#[derive(Clone)]
pub struct OrderingTerm {
    pub expr: Expression,
    pub order: Order,
}

#[derive(Copy, Clone)]
pub enum Order {
    Ascending,
    Descending,
}

pub type JoinClause = (JoinOperator, String, Option<JoinConstraint>);

pub enum FromClause {
    TableOrSubquery(Vec<String>),
    JoinClause(String, Option<Vec<JoinClause>>),
}

#[derive(Copy)]
pub enum JoinOperator {
    Inner,
    Natural,
}

#[derive(Clone)]
pub enum JoinConstraint {
    On(Expression),
}
