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
}
