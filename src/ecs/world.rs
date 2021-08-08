use std::vec;

pub struct World<TSystems> {
    pub systems: Vec<TSystems>,
}

impl<TSystems> World<TSystems> {
    pub fn new() -> Self {
        Self {
            systems: vec![],
        }
    }
}