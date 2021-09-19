use std::sync::Arc;

use winit::event_loop::ControlFlow;

use super::{EntityComponentSystem, System};

pub struct State {
}

impl System for State {
}

impl State {
    pub fn update(&mut self, ecs: Arc<EntityComponentSystem>, control_flow: &mut ControlFlow) {
    }
}