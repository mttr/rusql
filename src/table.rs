use parser::definitions::{LiteralValue, ColumnDef};

pub type TableEntry = Vec<LiteralValue>;
pub type TableHeader = Vec<ColumnDef>;

pub struct Table<'a> {
    pub header: TableHeader,
    pub entries: Vec<TableEntry>,
}

impl<'a> Table<'a> {
    pub fn get_column_def_by_name(&'a self, name: String) -> Option<&'a ColumnDef> {
        for column_def in self.header.iter().filter(|&cols| cols.name == name) {
            return Some(column_def);
        }
        None
    }

    pub fn get_column_index(&'a self, name: String) -> Option<uint> {
        for (i, _) in self.header.iter().filter(|&cols| cols.name == name).enumerate() {
            return Some(i);
        }
        None
    }

    pub fn has_entry(&'a self, pk: int) -> bool {
        let index = self.get_column_index("Id".to_string()).unwrap();

        for entry in self.entries.iter() {
            match entry[index] {
                LiteralValue::Integer(n) if n == pk => return true,
                _ => continue,
            }
        }

        false
    }
}
