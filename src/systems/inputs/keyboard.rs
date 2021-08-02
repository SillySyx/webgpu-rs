use winit::event::{VirtualKeyCode, ElementState};

use crate::ecs::System;

pub struct Keyboard {
    pressed_keys: Vec<VirtualKeyCode>,
}

impl System for Keyboard {
    fn update(_frame_time: u32) {
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed_keys: vec![],
        }
    }

    pub fn handle_input(&mut self, input: &winit::event::KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            if input.state == ElementState::Pressed {
                println!("Pressed {:?}", keycode);
                self.pressed_keys.push(keycode);
            }
            if input.state == ElementState::Released {
                println!("Released {:?}", keycode);
                if let Some(index) = self.pressed_keys.iter().position(|pressed_keycode| pressed_keycode == &keycode) {
                    self.pressed_keys.remove(index);
                }
            }
        }
    }
}
