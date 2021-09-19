use std::error::Error;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, Event, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

// https://bfnightly.bracketproductions.com/rustbook/chapter_2.html
// https://github.com/rg3dengine/rg3d/blob/9fa2d7d021bf16bb9a46215ec95ce83bd34b4ce1/src/engine/framework.rs

pub enum WindowMessage {
    Focused(bool),
    KeyboardInput(KeyboardInput),
    ModifiersChanged(ModifiersState),
    MouseInput(MouseButton, ElementState),
    MouseMoved(PhysicalPosition<f64>),
    MouseScrolled(MouseScrollDelta),
    Redraw,
    Resized(PhysicalSize<u32>),
}

pub struct Window {
    handle: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new();

        let handle = WindowBuilder::new()
            .with_title("test")
            .with_inner_size(PhysicalSize::new(1920, 1080))
            .with_fullscreen(None)
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            handle,
        })
    }

    pub fn run<F>(self, message_dispatcher: F) where F: Fn(WindowMessage, &mut ControlFlow) + 'static,
    {
        let window = self.handle;

        self.event_loop.run(move |event, _, control_flow| {
            if let Event::WindowEvent { ref event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let WindowEvent::CursorMoved { position, .. } = event {
                    message_dispatcher(WindowMessage::MouseMoved(*position), control_flow);
                }

                if let WindowEvent::Focused(focused) = event {
                    message_dispatcher(WindowMessage::Focused(*focused), control_flow);
                }

                if let WindowEvent::KeyboardInput { input, .. } = event {
                    message_dispatcher(WindowMessage::KeyboardInput(*input), control_flow);
                }

                if let WindowEvent::MouseInput { button, state, .. } = event {
                    message_dispatcher(WindowMessage::MouseInput(*button, *state), control_flow);
                }

                if let WindowEvent::MouseWheel { delta, .. } = event {
                    message_dispatcher(WindowMessage::MouseScrolled(*delta), control_flow);
                }

                if let WindowEvent::ModifiersChanged(state) = event {
                    message_dispatcher(WindowMessage::ModifiersChanged(*state), control_flow);
                }

                if let WindowEvent::Resized(physical_size) = event {
                    message_dispatcher(WindowMessage::Resized(*physical_size), control_flow);
                }

                if let WindowEvent::ScaleFactorChanged { new_inner_size, .. } = event {
                    message_dispatcher(WindowMessage::Resized(**new_inner_size), control_flow);
                }
            }

            if let Event::RedrawRequested(_) = event {
                message_dispatcher(WindowMessage::Redraw, control_flow);
            }

            if let Event::MainEventsCleared = event {
                window.request_redraw();
            }
        });
    }
}