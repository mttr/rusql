use definitions::{LiteralValue, ColumnDef, ColumnConstraint};

use std::collections::BTreeMap;
use std::fmt;

pub type TableRow = Vec<LiteralValue>;
pub type TableHeader = Vec<ColumnDef>;
pub type PkType = uint;

pub struct RowFormat<'a>(pub &'a TableRow);

pub struct Table {
    pub name: String,
    pub header: TableHeader,
    pub data: BTreeMap<PkType, TableRow>,
    pub pk: Option<PkType>,
}

impl Table {
    pub fn new_result_table(header: TableHeader) -> Table {
        Table {
            name: "".to_string(),
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

    pub fn has_row(&self, pk: PkType) -> bool {
        self.data.contains_key(&pk)
    }

    pub fn assert_size(&self) {
        let header_size = self.header.len();

        for row in self.data.values() {
            assert!(row.len() == header_size);
        }
    }

    pub fn add_column(&mut self, column_def: &ColumnDef) {
        self.header.push(column_def.clone());

        for (_, row) in self.data.iter_mut() {
            row.push(LiteralValue::Null);
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
                let mut row = Vec::from_elem(self.header.len(), LiteralValue::Null);

                for (name, data) in column_names.iter().zip(column_data.iter()) {
                    row[self.get_column_index(name.clone()).unwrap()] = data.clone();
                }

                self.push_row(row);
            } else {
                self.push_row(column_data.clone());
            }
        }
    }

    pub fn push_row(&mut self, row: TableRow) {
        if let Some(i) = self.pk {
            let pk = row[i].clone().to_uint();
            self.data.insert(pk, row);
        } else {
            let len = self.data.len();
            self.data.insert(len, row);
        }
    }

    pub fn delete_where(&mut self, f: |row: &TableRow| -> bool) {
        let mut keys: Vec<PkType> = Vec::new();

        for (key, row) in self.data.iter() {
            if !f(row) {
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

impl<'a> fmt::Show for RowFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for column in self.0.iter() {
            write!(f, "{} | ", column).ok();
        }
        Ok(())
    }
}

pub fn get_column(name: &String, row: &TableRow, head: &TableHeader, offset: Option<uint>) -> LiteralValue {
    let x = if let Some(x) = offset { x } else { 0 };
    row[head.iter().position(|ref def| def.name == *name).unwrap() + x].clone()
}
