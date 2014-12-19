pub enum RusqlStatement {
    CreateTable(TableDef),
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

pub struct TableDef {
    pub table_name: String,
    pub columns: Vec<ColumnDef>,
}

pub struct ColumnDef {
    pub column_type: Option<ColumnType>,
    pub column_constraints: Vec<ColumnConstraint>,
}
