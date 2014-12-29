use std::fmt;

pub enum RusqlStatement {
    AlterTable(AlterTableDef),
    CreateTable(TableDef),
    Delete(DeleteDef),
    DropTable(DropTableDef),
    Insert(InsertDef),
    Select(SelectDef),
    Update(UpdateDef),
}

#[deriving(Copy, Clone, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    Text,
}

#[deriving(Copy, Clone, PartialEq, Eq)]
pub enum ColumnConstraint {
    PrimaryKey,
}

#[deriving(Clone, PartialEq)]
pub enum LiteralValue {
    Integer(int),
    Text(String),
    Real(f64),
    Boolean(bool),
    Null,
}

impl LiteralValue {
    pub fn to_uint(&self) -> uint {
        match self {
            &LiteralValue::Integer(i) => i as uint,
            _ => 0, // FIXME ???
        }
    }
    pub fn to_int(&self) -> int {
        match self {
            &LiteralValue::Integer(i) => i,
            _ => 0, // FIXME ???
        }
    }

    pub fn neg(&self) -> LiteralValue {
        match self {
            &LiteralValue::Integer(i) => LiteralValue::Integer(-i),
            _ => self.clone(),
        }
    }
}

impl fmt::Show for LiteralValue {
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

pub struct TableDef {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
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

#[deriving(Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub column_type: Option<ColumnType>,
    pub column_constraints: Vec<ColumnConstraint>,
}

pub struct SelectDef {
    pub result_column: ResultColumn,
    pub table_or_subquery: Option<Vec<String>>,
    pub where_expr: Option<Expression>,
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

#[deriving(Show, Clone)]
pub enum Expression {
    LiteralValue(LiteralValue),
    TableName((String, Box<Expression>)),
    ColumnName(String),
    BinaryOperator((BinaryOperator, Box<Expression>, Box<Expression>)),
    UnaryOperator((UnaryOperator, Box<Expression>)),
}

#[deriving(Copy, Show, Clone)]
pub enum BinaryOperator {
    Equals,
    Plus,
    Minus,
}

impl BinaryOperator {
    pub fn neg(&self) -> BinaryOperator {
        match *self {
            BinaryOperator::Plus => BinaryOperator::Minus,
            BinaryOperator::Minus => BinaryOperator::Plus,
            _ => *self
        }
    }
}

#[deriving(Copy, Show, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl UnaryOperator {
    pub fn neg(&self) -> UnaryOperator {
        match *self {
            UnaryOperator::Plus => UnaryOperator::Minus,
            UnaryOperator::Minus => UnaryOperator::Plus,
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
