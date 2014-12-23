use parser::definitions::TableDef;
use table::Table;

use std::collections::BTreeMap;

pub struct Rusql<'a> {
    pub map: BTreeMap<String, Table<'a>>,
}


impl<'a> Rusql<'a> {
    pub fn new() -> Rusql<'a> {
        return Rusql {
            map: BTreeMap::new(),
        };
    }

    pub fn rename_table(&mut self, old_name: &String, new_name: &String) {
        let table = self.map.remove(old_name.as_slice()).unwrap();
        self.map.insert(new_name.clone(), table);
    }

    pub fn get_table(&self, name: &String) -> &Table<'a> {
        self.map.get(name.as_slice()).unwrap()
    }

    pub fn get_mut_table(&mut self, name: &String) -> &mut Table<'a> {
        self.map.get_mut(name.as_slice()).unwrap()
    }

    pub fn create_table(&mut self, table_def: &TableDef) {
        self.map.insert(table_def.table_name.clone(), Table {
            header: table_def.columns.clone(),
            entries: Vec::new(),
        });
    }

    pub fn drop_table(&mut self, name: &String) {
        self.map.remove(name.as_slice());
    }
}
