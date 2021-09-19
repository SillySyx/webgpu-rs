use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use super::System;

pub struct Keyboard {
    pressed_keys: Vec<VirtualKeyCode>,
}

impl System for Keyboard {
}

impl Keyboard {
    pub fn handle_input(&mut self, input: &KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            if input.state == ElementState::Pressed {
                self.pressed_keys.push(keycode);
            }
            if input.state == ElementState::Released {
                if let Some(index) = self.pressed_keys.iter().position(|pressed_keycode| pressed_keycode == &keycode) {
                    self.pressed_keys.remove(index);
                }
            }
        }
    }
}