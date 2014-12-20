use table::Table;

use std::collections::TreeMap;

pub struct Rusql<'a> {
    pub map: TreeMap<String, Table<'a>>,
}


impl<'a> Rusql<'a> {
    pub fn new() -> Rusql<'a> {
        return Rusql {
            map: TreeMap::new(),
        };
    }
}
