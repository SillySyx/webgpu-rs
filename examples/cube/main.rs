use winit::dpi::PhysicalSize;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode};

#[async_std::main]
async fn main() {
    let event_loop = EventLoop::new();

    let window = match WindowBuilder::new()
        .with_title("Cube example")
        .with_inner_size(PhysicalSize::new(1980, 1080))
        .build(&event_loop) {
        Ok(window) => window,
        Err(_) => panic!("No window created!"),
    };

    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };

    let adapter_options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
    };
    let adapter = match instance.request_adapter(&adapter_options).await {
        Some(adapter) => adapter,
        None => panic!("No adapter found!"),
    };

    let device_descriptor = wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
    };
    let trace_path = None;
    let (device, queue) = match adapter.request_device(&device_descriptor, trace_path).await {
        Ok((device, queue)) => (device, queue),
        Err(_) => panic!("Failed to create device and queue"),
    };

    let size = window.inner_size();
    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        flags: wgpu::ShaderFlags::all(),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    };
    let shader_module = device.create_shader_module(&shader_module_descriptor);

    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    };
    let render_pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

    let fragment_state = wgpu::FragmentState { // 3.
        module: &shader_module,
        entry_point: "main",
        targets: &[wgpu::ColorTargetState { // 4.
            format: swap_chain_descriptor.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrite::ALL,
        }],
    };

    let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "main", // 1.
            buffers: &[], // 2.
        },
        fragment: Some(fragment_state),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, // 2.
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLAMPING
            clamp_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None, // 1.
        multisample: wgpu::MultisampleState {
            count: 1, // 2.
            mask: !0, // 3.
            alpha_to_coverage_enabled: false, // 4.
        },
    };
    let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id } => {
                if window_id != window.id() {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        match input {
                            KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. } => *control_flow = ControlFlow::Exit,
                            KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::F1), .. } => {
                                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                            }
                            KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::F2), .. } => {
                                window.set_fullscreen(None);
                            }
                            _ => {}
                        }
                    },
                    WindowEvent::Resized(physical_size) => {
                        let size = *physical_size;
                        swap_chain_descriptor.width = size.width;
                        swap_chain_descriptor.height = size.height;
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let size = **new_inner_size;
                        swap_chain_descriptor.width = size.width;
                        swap_chain_descriptor.height = size.height;
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
                    },
                    _ => {},
                }
            },
            Event::RedrawRequested(_) => {
                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(wgpu::SwapChainError::Lost) => {
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
                        swap_chain.get_current_frame().unwrap()
                    },
                    Err(wgpu::SwapChainError::Outdated) => {
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
                        swap_chain.get_current_frame().unwrap()
                    },
                    Err(_) => panic!("failed to get frame"),
                };

                let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                };
                let mut encoder = device.create_command_encoder(&command_encoder_descriptor);
                {
                    let render_pass_descriptor = wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[
                            wgpu::RenderPassColorAttachment {
                                view: &frame.output.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.1,
                                        g: 0.2,
                                        b: 0.3,
                                        a: 1.0,
                                    }),
                                    store: true,
                                }
                            }
                        ],
                        depth_stencil_attachment: None,
                    };
                    let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw(0..3, 0..1);
                }

                let command_buffer = encoder.finish();
                queue.submit(std::iter::once(command_buffer));
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            },
            _ => {},
        }
    });
}