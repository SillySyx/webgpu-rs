use anymap::AnyMap;

pub trait System {
    fn update(frame_time: u32);
}

pub struct SystemsRegistry {
    pub systems: AnyMap,
}

impl SystemsRegistry {
    pub fn new() -> Self {
        Self {
            systems: AnyMap::new(),
        }
    }

    pub fn register_system<T: System + 'static>(&mut self, system: T) {
        self.systems.insert(system);
    }

    pub fn get_system<T: System + 'static>(&mut self) -> Option<&T> {
        self.systems.get()
    }

    pub fn get_system_mut<T: System + 'static>(&mut self) -> Option<&mut T> {
        self.systems.get_mut()
    }
}
