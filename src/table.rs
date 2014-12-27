use definitions::{LiteralValue, ColumnDef, ColumnConstraint};

use std::collections::BTreeMap;

pub type TableEntry = Vec<LiteralValue>;
pub type TableHeader = Vec<ColumnDef>;

pub struct Table {
    pub header: TableHeader,
    pub data: BTreeMap<uint, TableEntry>,
    pub pk: Option<uint>,
}

impl Table {
    pub fn new_result_table(header: TableHeader) -> Table {
        Table {
            header: header,
            data: BTreeMap::new(),
            pk: None,
        }
    }
    pub fn get_column_def_by_name(&self, name: String) -> Option<&ColumnDef> {
        self.header.iter().find(|&cols| cols.name == name)
    }

    pub fn get_column_index(&self, name: String) -> Option<uint> {
        self.header.iter().position(|ref cols| cols.name == name)
    }

    pub fn has_entry(&self, pk: uint) -> bool {
        self.data.contains_key(&pk)
    }

    pub fn assert_size(&self) {
        let header_size = self.header.len();

        for entry in self.data.values() {
            assert!(entry.len() == header_size);
        }
    }

    pub fn add_column(&mut self, column_def: &ColumnDef) {
        self.header.push(column_def.clone());

        for (_, entry) in self.data.iter_mut() {
            entry.push(LiteralValue::Null);
        }
    }

    pub fn add_columns(&mut self, column_defs: Vec<&ColumnDef>) {
        for def in column_defs.iter() {
            self.add_column(*def);
        }
    }

    pub fn insert(&mut self, column_data: &Vec<Vec<LiteralValue>>,
                  specified_columns: &Option<Vec<String>>) {
        for column_data in column_data.iter() {
            if let &Some(ref column_names) = specified_columns {
                assert!(column_names.len() == column_data.len());
                let mut entry = Vec::from_elem(self.header.len(), LiteralValue::Null);

                for (name, data) in column_names.iter().zip(column_data.iter()) {
                    entry[self.get_column_index(name.clone()).unwrap()] = data.clone();
                }

                self.push_entry(entry);
            } else {
                self.push_entry(column_data.clone());
            }
        }
    }

    pub fn push_entry(&mut self, entry: TableEntry) {
        if let Some(i) = self.pk {
            let pk = entry[i].clone().to_uint();
            self.data.insert(pk, entry);
        } else {
            let len = self.data.len();
            self.data.insert(len, entry);
        }
    }

    pub fn delete_where(&mut self, f: |entry: &TableEntry| -> bool) {
        let mut keys: Vec<uint> = Vec::new();

        for (key, entry) in self.data.iter() {
            if !f(entry) {
                continue;
            }
            keys.push(key.clone());
        }

        for key in keys.iter() {
            self.data.remove(key);
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn process_constraints(&mut self) {
        for (i, column) in self.header.iter().enumerate() {
            for constraint in column.column_constraints.iter() {
                match constraint {
                    &ColumnConstraint::PrimaryKey => self.pk = Some(i),
                }
            }
        }
    }
}

pub fn get_column(name: &String, entry: &TableEntry, head: &TableHeader) -> LiteralValue {
    entry[head.iter().position(|ref def| def.name == *name).unwrap()].clone()
}
