use anymap::AnyMap;

use super::System;

pub trait ECS: 'static {
    fn new() -> Self where Self: Sized;

    fn add_system<T: System>(&mut self, system: T);
    fn get_system_mut<T: System>(&mut self) -> Option<&mut T>;
    fn get_system<T: System>(&self) -> Option<&T>;
}

pub struct EntityComponentSystem {
    pub systems: AnyMap,
}

impl ECS for EntityComponentSystem {
    fn new() -> Self where Self: Sized {
        Self {
            systems: AnyMap::new(),
        }
    }

    fn add_system<T: System>(&mut self, system: T) {
        self.systems.insert(system);
    }

    fn get_system_mut<T: System>(&mut self) -> Option<&mut T> {
        self.systems.get_mut()
    }

    fn get_system<T: System>(&self) -> Option<&T> {
        self.systems.get()
    }
}