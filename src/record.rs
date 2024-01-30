use std::collections::{btree_map::Iter, BTreeMap};

use crate::casing::Casing;

#[derive(Debug, Clone)]
pub struct Record {
    pub pos: usize,
    pub len: usize,
    pub casing: Casing,
}

#[derive(Debug)]
pub struct Records {
    pub map: BTreeMap<usize, Record>,
}

impl Records {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn try_insert(&mut self, pos: usize, len: usize, casing: Casing) -> Result<(), String> {
        if self.map.contains_key(&pos) {
            return Err("Key already present".into());
        }

        self.map.insert(pos, Record { pos, len, casing });

        Ok(())
    }

    pub fn iter(&self) -> Iter<'_, usize, Record> {
        self.map.iter()
    }

    // pub fn iter_mut(&mut self) -> IterMut<'_, usize, Record> {
    //     self.map.iter_mut()
    // }

    // pub fn into_iter(self) -> IntoIter<usize, Record> {
    //     self.map.into_iter()
    // }
}
