use std::collections::{HashMap, VecDeque};

#[derive(Default, Debug)]
pub struct Queue {
    list: VecDeque<u32>,
    map: HashMap<u32, bool>,
}

impl Queue {
    pub fn new(list: VecDeque<u32>, map: HashMap<u32, bool>) -> Self {
        Self { list, map }
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push(&mut self, item: u32) {
        if self.map.contains_key(&item) {
            return;
        }
        self.map.insert(item, true);
        self.list.push_back(item);
    }
    pub fn pop(&mut self) -> Option<u32> {
        let pc = self.list.pop_front();
        if let Some(pc) = pc {
            self.map.remove(&pc);
        }
        pc
    }
}
