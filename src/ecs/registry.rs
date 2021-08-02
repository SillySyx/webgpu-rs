use std::cell::RefCell;
use lazy_static::*;

use super::{ComponentRegistry, SystemsRegistry};

pub struct Registry {
    pub components: RefCell<ComponentRegistry>,
    pub systems: RefCell<SystemsRegistry>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            components: RefCell::new(ComponentRegistry::new()),
            systems: RefCell::new(SystemsRegistry::new()),
        }
    }
}

unsafe impl Sync for Registry { }

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
}