use parser::definitions::{LiteralValue, ColumnDef};

pub type TableEntry = Vec<LiteralValue>;
pub type TableHeader = Vec<ColumnDef>;

pub struct Table<'a> {
    pub header: TableHeader,
    pub entries: Vec<TableEntry>,
}

impl<'a> Table<'a> {
    pub fn get_column_def_by_name(&'a self, name: String) -> Option<&'a ColumnDef> {
        self.header.iter().find(|&cols| cols.name == name)
    }

    pub fn get_column_index(&'a self, name: String) -> Option<uint> {
        self.header.iter().position(|ref cols| cols.name == name)
    }

    pub fn has_entry(&'a self, pk: int) -> bool {
        let index = self.get_column_index("Id".to_string()).unwrap();

        self.entries.iter().any(|entry| entry[index] == LiteralValue::Integer(pk))
    }
}

pub fn get_column(name: &String, entry: &TableEntry, head: &TableHeader) -> LiteralValue {
    entry[head.iter().position(|ref def| def.name == *name).unwrap()].clone()
}
