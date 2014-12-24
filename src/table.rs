use definitions::{LiteralValue, ColumnDef};

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

    pub fn assert_size(&'a self) {
        let header_size = self.header.len();

        for entry in self.entries.iter() {
            assert!(entry.len() == header_size);
        }
    }

    pub fn add_column(&'a mut self, column_def: &ColumnDef) {
        self.header.push(column_def.clone());

        for entry in self.entries.iter_mut() {
            entry.push(LiteralValue::Null);
        }
    }

    pub fn insert(&'a mut self, column_data: &Vec<Vec<LiteralValue>>, 
                  specified_columns: &Option<Vec<String>>) {
        for column_data in column_data.iter() {
            if let &Some(ref column_names) = specified_columns {
                assert!(column_names.len() == column_data.len());
                let mut entry = Vec::from_elem(self.header.len(), LiteralValue::Null);

                for (name, data) in column_names.iter().zip(column_data.iter()) {
                    entry[self.get_column_index(name.clone()).unwrap()] = data.clone();
                }

                self.entries.push(entry);
            } else {
                self.entries.push(column_data.clone());
            }
        }
    }

    pub fn delete_where(&'a mut self, f: |entry: &TableEntry| -> bool) {
        self.entries.retain(|entry| !f(entry));
    }

    pub fn clear(&'a mut self) {
        self.entries.clear();
    }
}

pub fn get_column(name: &String, entry: &TableEntry, head: &TableHeader) -> LiteralValue {
    entry[head.iter().position(|ref def| def.name == *name).unwrap()].clone()
}
