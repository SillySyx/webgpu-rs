mod ecs;

use std::{error::Error, sync::Arc};

use ecs::{ECS, EntityComponentSystem, Window, WindowMessage};
use winit::event_loop::ControlFlow;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // let ecs = Arc::new(EntityComponentSystem::new());

    let window = Window::new()?;

    window.run(|message, control_flow| {
        if let WindowMessage::KeyboardInput(input) = message {
            if let Some(winit::event::VirtualKeyCode::Escape) = input.virtual_keycode {
                *control_flow = ControlFlow::Exit;
            }
        }
    });

    Ok(())
}