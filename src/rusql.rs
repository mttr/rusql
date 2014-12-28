use definitions::TableDef;
use table::Table;

use std::collections::BTreeMap;

pub struct Rusql {
    pub map: BTreeMap<String, Table>,
}


impl Rusql {
    pub fn new() -> Rusql {
        return Rusql {
            map: BTreeMap::new(),
        };
    }

    pub fn rename_table(&mut self, old_name: &String, new_name: String) {
        let table = self.map.remove(old_name.as_slice()).unwrap();
        self.map.insert(new_name, table);
    }

    pub fn get_table(&self, name: &String) -> &Table {
        self.map.get(name.as_slice()).unwrap()
    }

    pub fn get_mut_table(&mut self, name: &String) -> &mut Table {
        self.map.get_mut(name.as_slice()).unwrap()
    }

    pub fn create_table(&mut self, table_def: TableDef) {
        let mut table = Table {
            name: table_def.table_name,
            header: table_def.columns,
            data: BTreeMap::new(),
            pk: None,
        };
        table.process_constraints();
        self.map.insert(table.name.clone(), table);
    }

    pub fn drop_table(&mut self, name: &String) {
        self.map.remove(name.as_slice());
    }
}
