pub enum RusqlStatement {
    CreateTable(TableDef),
    Insert(InsertDef),
    Select(SelectDef),
}

#[deriving(Copy)]
pub enum ColumnType {
    Integer,
    Text,
}

#[deriving(Copy)]
pub enum ColumnConstraint {
    PrimaryKey,
}

#[deriving(Show)]
pub enum LiteralValue {
    Integer(int),
    Text(String),
    Real(f64),
    Null,
}

pub struct TableDef {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
}

#[deriving(Copy)]
pub enum ResultColumn {
    Asterisk,
}

pub struct InsertDef {
    pub table_name: String,
    pub column_data: Vec<LiteralValue>,
}

pub struct ColumnDef {
    pub name: String,
    pub column_type: Option<ColumnType>,
    pub column_constraints: Vec<ColumnConstraint>,
}

pub struct SelectDef {
    pub result_column: ResultColumn,
    pub table_or_subquery: Vec<String>,
}
