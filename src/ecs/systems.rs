use core::any::*;
use std::{borrow::BorrowMut, collections::HashMap};

pub type SystemMap = HashMap<TypeId, Box<dyn System>>;

pub trait System: Any {
    fn update(&mut self, frame_time: u32);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct SystemsRegistry {
    systems: SystemMap,
}

impl SystemsRegistry {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }

    pub fn register_system<T: System>(&mut self, system: T) {
        let id = TypeId::of::<T>();
        self.systems.insert(id, Box::new(system));
    }

    pub fn get_system<T: System>(&self) -> Option<&T> {
        if let Some(system) = self.systems.get(&TypeId::of::<T>()) {
            return system.as_any().downcast_ref::<T>();
        }
        None
    }

    pub fn get_system_mut<T: System>(&mut self) -> Option<&mut T> {
        if let Some(system) = self.systems.get_mut(&TypeId::of::<T>()) {
            return system.as_any_mut().downcast_mut();
        }
        None
    }

    pub fn get_systems_mut(&mut self) -> &mut SystemMap {
        self.systems.borrow_mut()
    }

    // pub fn update(&mut self) {
    //     let values = self.systems.values_mut();
    //     let systems = self.systems.borrow_mut();
    //     for system in values {
    //         system.update(systems, 0);
    //     }

    //     let test = self.systems.iter_mut().enumerate();
    //     for (_, (_, system)) in test {
    //         system.update(0);
    //     }

    //     let iter = self.systems.iter_mut();
    //     let systems = self.systems.borrow_mut();
    //     for (_, system) in iter {
    //         system.update(&systems, 0);
    //     }
    // }
}
