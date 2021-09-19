use std::error::Error;
use std::sync::Arc;

use anymap::AnyMap;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::monitor::MonitorHandle;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Fullscreen, WindowBuilder};

use crate::window::{WindowConfiguration, WindowModes};

pub struct Window {
    handle: winit::window::Window,
    event_loop: EventLoop<()>,
    pub ecs: EntityComponentSystem,
}

pub trait System { }

pub trait SystemRegistry {
    fn get_system_mut<T: System + 'static>(&mut self) -> Option<&mut T>;
    fn add_system<T: System + 'static>(&mut self, system: T);
}

pub struct EntityComponentSystem {
    systems: AnyMap,
}

impl SystemRegistry for EntityComponentSystem {
    fn get_system_mut<T: System + 'static>(&mut self) -> Option<&mut T> {
        self.systems.get_mut()
    }

    fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.insert(system);
    }
}

// https://bfnightly.bracketproductions.com/rustbook/chapter_2.html

pub struct Keyboard {
    pressed_keys: Vec<VirtualKeyCode>,
}

impl System for Keyboard { }

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

impl Window {
    pub fn new(config: WindowConfiguration) -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new();
        let monitor = match event_loop.primary_monitor() {
            Some(monitor) => monitor,
            None => event_loop.available_monitors().next().unwrap(),
        };

        let fullscreen = select_fullscreen_from_configuration(monitor, &config);

        let handle = WindowBuilder::new()
            .with_title(&config.title)
            .with_inner_size(PhysicalSize::new(config.width, config.height))
            .with_fullscreen(fullscreen)
            .build(&event_loop)?;

        let mut ecs = EntityComponentSystem {
            systems: AnyMap::new(),
        };
        ecs.add_system(Keyboard {
            pressed_keys: vec![]
        });

        Ok(Self { 
            event_loop, 
            handle,
            ecs,
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
        let mut ecs = self.ecs;

        // let mut frame_time = FrameTime::new(self.target_frame_time);

        self.event_loop.run(move |event, _, control_flow| {
            if let Event::WindowEvent { ref event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let WindowEvent::Resized(_physical_size) = event {
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

                    if let Some(keyboard) = ecs.get_system_mut::<Keyboard>() {
                        keyboard.handle_input(input);
                    }
                }
            }

            if let Event::RedrawRequested(_) = event {
                // let frame_ms = frame_time
                //     .update()
                //     .expect("Failed to update frame time");

                // frame(frame_ms);

                // if let Some(sleep_duration) = frame_time.calc_sleep_duration() {
                //     sleep(sleep_duration);
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
            let video_mode = monitor.video_modes().find(|_video_mode| false)?;

            Some(Fullscreen::Exclusive(video_mode))
        }
    }
}