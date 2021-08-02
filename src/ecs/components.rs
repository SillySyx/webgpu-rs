use std::collections::HashMap;
use anymap::AnyMap;

pub trait Component {
}

pub type EntityId = usize;
pub type EntityComponents = AnyMap;

pub struct ComponentRegistry {
    pub components: HashMap<EntityId, EntityComponents>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: Component>(&mut self, entity: EntityId, component: T) {
    }

    pub fn get_component<T: Component>(&mut self, entity: EntityId) -> Option<&T> {
        None
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        None
    }
}