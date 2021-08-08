use std::error::Error;
use winit::dpi::PhysicalSize;
use winit::window::{Fullscreen, WindowBuilder};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use winit::monitor::MonitorHandle;

use crate::{
    ecs::SystemsRegistry,
    systems::Keyboard,
    window::{WindowConfiguration, WindowModes},
};

pub struct Window {
    handle: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = WindowConfiguration::new();

        let event_loop = EventLoop::new();
        let monitor = match event_loop.primary_monitor() {
            Some(monitor) => monitor,
            None => event_loop.available_monitors().next().unwrap()
        };

        let fullscreen = select_fullscreen_from_configuration(monitor, &config);

        let handle = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_fullscreen(fullscreen)
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            handle,
        })
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.handle.inner_size()
    }

    pub fn window_handle(&self) -> &winit::window::Window {
        &self.handle
    }

    pub fn run(self) {
        let window = self.handle;

        self.event_loop.run(move |event, _, control_flow| {
            if let Event::WindowEvent { ref event, window_id } = event {
                if window.id() != window_id {
                    return;
                }

                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let WindowEvent::Resized(physical_size) = event {
                    // let mut systems = REGISTRY.systems.borrow_mut();
                    // if let Some(renderer) = systems.get_system_mut::<Renderer>() {
                    //     renderer.resize(*physical_size);
                    // }
                }

                if let WindowEvent::ScaleFactorChanged { new_inner_size, .. } = event {
                    // let mut systems = REGISTRY.systems.borrow_mut();
                    // if let Some(renderer) = systems.get_system_mut::<Renderer>() {
                    //     renderer.resize(**new_inner_size);
                    // }
                }

                if let WindowEvent::KeyboardInput { input, .. } = event {
                    // let mut systems = systems.borrow_mut();
                    // if let Some(keyboard) = systems.get_system_mut::<Keyboard>() {
                    //     keyboard.handle_input(input);
                    // }
                }
            }

            if let Event::RedrawRequested(_) = event {
                // fps limit!?

                // let mut reg = systems.borrow_mut();
                // let mut systems = systems.get_systems_mut();
                // for (_, system) in systems.iter_mut() {
                //     system.update(&mut systems, 0);
                // }
                
                // if let Some(renderer) = systems.get_system_mut::<Renderer>() {
                //     renderer.draw();
                // }
            }

            if let Event::MainEventsCleared = event {
                window.request_redraw();
            }
        });
    }
}

fn select_fullscreen_from_configuration(monitor: MonitorHandle, configuration: &WindowConfiguration) -> Option<Fullscreen> {
    match configuration.window_mode {
        WindowModes::Window => None,
        WindowModes::Borderless => Some(Fullscreen::Borderless(None)),
        WindowModes::Exclusive => {
            let video_mode = monitor
                .video_modes()
                .find(|video_mode| {
                    false
                })?;
                
            Some(Fullscreen::Exclusive(video_mode))
        },
    }
}