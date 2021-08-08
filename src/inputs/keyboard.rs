use winit::event::{VirtualKeyCode, ElementState};

pub struct Keyboard {
    pressed_keys: Vec<VirtualKeyCode>,
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
                self.pressed_keys.push(keycode);
            }
            if input.state == ElementState::Released {
                if let Some(index) = self.pressed_keys.iter().position(|pressed_keycode| pressed_keycode == &keycode) {
                    self.pressed_keys.remove(index);
                }
            }
        }
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter().any(|pressed_key| pressed_key == &key)
    }
}
