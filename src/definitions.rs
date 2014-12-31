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

            /*if x < y { return Less }
            else if x > y { return Greater }
            else { return Equal }*/
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

    fn int_add(&self, x: int, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x + i),
            _ => LiteralValue::Null,
        }
    }

    fn int_sub(&self, x: int, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x - i),
            _ => LiteralValue::Null,
        }
    }

    fn int_mul(&self, x: int, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x * i),
            _ => LiteralValue::Null,
        }
    }

    fn int_div(&self, x: int, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x / i),
            _ => LiteralValue::Null,
        }
    }

    fn int_rem(&self, x: int, rhs: LiteralValue) -> LiteralValue {
        match rhs {
            LiteralValue::Integer(i) => LiteralValue::Integer(x % i),
            _ => LiteralValue::Null,
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

impl Add<LiteralValue, LiteralValue> for LiteralValue {
    fn add(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_add(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Sub<LiteralValue, LiteralValue> for LiteralValue {
    fn sub(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_sub(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Mul<LiteralValue, LiteralValue> for LiteralValue {
    fn mul(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_mul(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Div<LiteralValue, LiteralValue> for LiteralValue {
    fn div(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_div(i, rhs),
            _ => LiteralValue::Null,
        }
    }
}

impl Rem<LiteralValue, LiteralValue> for LiteralValue {
    fn rem(self, rhs: LiteralValue) -> LiteralValue {
        match self {
            LiteralValue::Integer(i) => self.int_rem(i, rhs),
            _ => LiteralValue::Null,
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

#[deriving(Copy, Show, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Null,
    Mult,
    Divide,
    Modulo,
    Plus,
    Minus,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    Equals,
    NotEquals,
    And,
    Or,
}

impl PartialOrd for BinaryOperator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ord_val() < other.ord_val() { Some(Less) }
        else if self.ord_val() > other.ord_val() { Some(Greater) }
        else { Some(Equal) }
    }
}

impl BinaryOperator {
    pub fn neg(&self) -> BinaryOperator {
        match *self {
            BinaryOperator::Plus => BinaryOperator::Minus,
            BinaryOperator::Minus => BinaryOperator::Plus,
            _ => *self
        }
    }

    fn ord_val(&self) -> uint {
        match *self {
            BinaryOperator::Null => 0,
            BinaryOperator::Mult | BinaryOperator::Divide | BinaryOperator::Modulo => 1,
            BinaryOperator::Plus | BinaryOperator::Minus => 2,
            BinaryOperator::Less | BinaryOperator::LessEq
                | BinaryOperator::Greater | BinaryOperator::GreaterEq => 3,
            BinaryOperator::Equals | BinaryOperator::NotEquals => 4,
            BinaryOperator::And => 5,
            BinaryOperator::Or => 6,
        }
    }
}

#[deriving(Copy, Show, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
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
