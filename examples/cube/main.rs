use std::error::Error;
use winit::{
    dpi::PhysicalSize, 
    event::{Event, VirtualKeyCode, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}, 
    window::WindowBuilder
};
use wgpu::util::DeviceExt;

use webgpu::{
    inputs::Keyboard, 
    transforms::opengl_to_wgpu_conversion_matrix
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    model_matrix: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = match WindowBuilder::new()
        .with_title("Cube example")
        .with_inner_size(PhysicalSize::new(1980, 1080))
        .build(&event_loop) {
        Ok(window) => window,
        Err(_) => panic!("No window created!"),
    };
    let mut keyboard = Keyboard::new();

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

    let verticies = vec![
        Vertex { position: [ 1.0,  1.0, -1.0], color: [1.0, 1.0, 0.0] },
        Vertex { position: [ 1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 1.0,  1.0,  1.0], color: [1.0, 1.0, 1.0] },
        Vertex { position: [ 1.0, -1.0,  1.0], color: [0.0, 1.0, 1.0] },
        Vertex { position: [-1.0,  1.0, -1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 1.0] },
        Vertex { position: [-1.0,  1.0,  1.0], color: [0.0, 0.0, 0.0] },
        Vertex { position: [-1.0, -1.0,  1.0], color: [0.5, 0.5, 0.5] },
    ];
    let vertex_buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&verticies),
        usage: wgpu::BufferUsage::VERTEX,
    };
    let vertex_buffer = device.create_buffer_init(&vertex_buffer_descriptor);
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, 
        step_mode: wgpu::InputStepMode::Vertex, 
        attributes: &[ 
            wgpu::VertexAttribute {
                offset: 0, 
                shader_location: 0, 
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            }
        ],
    };

    let indices: Vec<u16> = vec![
        4, 2, 0,
        2, 7, 3,
        6, 5, 7,
        1, 7, 5,
        0, 3, 1,
        4, 1, 5,
        4, 6, 2,
        2, 6, 7,
        6, 4, 5,
        1, 3, 7,
        0, 2, 3,
        4, 0, 1,
    ];
    let index_buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsage::INDEX,
    };
    let index_buffer = device.create_buffer_init(&index_buffer_descriptor);
    let num_indices = indices.len() as u32;

    let eye = cgmath::Point3::<f32>::new(0.0, 2.0, 5.0);
    let target = cgmath::Point3::<f32>::new(0.0, 0.0, 0.0);
    let up = cgmath::Vector3::unit_y();
    let view_matrix = cgmath::Matrix4::look_at_rh(eye, target, up);

    let vertical_fov = cgmath::Deg(45.0);
    let aspect_ratio = size.width as f32 / size.height as f32;
    let near = 0.1;
    let far = 100.0;
    let projection_matrix = cgmath::perspective(vertical_fov, aspect_ratio, near, far);

    let view_projection_matrix = opengl_to_wgpu_conversion_matrix() * projection_matrix * view_matrix;
    let uniforms = Uniforms {
        view_proj: view_projection_matrix.into(),
    };
    let uniform_data = vec![uniforms];
    let uniform_buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&uniform_data),
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    };
    let uniform_buffer = device.create_buffer_init(&uniform_buffer_descriptor);

    let uniform_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
        label: Some("uniform_bind_group_layout"),
    };
    let uniform_bind_group_layout = device.create_bind_group_layout(&uniform_bind_group_layout_descriptor);
    
    let uniform_bind_group_descriptor = wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }
        ],
        label: Some("uniform_bind_group"),
    };
    let uniform_bind_group = device.create_bind_group(&uniform_bind_group_descriptor);

    let model_position1 = cgmath::Vector3::new(0.0, 0.0, 0.0);
    let model_position2 = cgmath::Vector3::new(0.0, 0.0, -5.0);
    let model_position3 = cgmath::Vector3::new(5.0, 0.0, -5.0);
    let model_position4 = cgmath::Vector3::new(-5.0, 0.0, -5.0);

    let model_rotation1 = cgmath::Vector3::new(0.0, 0.0, 0.0);
    let model_rotation2 = cgmath::Vector3::new(0.0, 180.0, 0.0);
    let model_rotation3 = cgmath::Vector3::new(0.0, -90.0, 0.0);
    let model_rotation4 = cgmath::Vector3::new(0.0, 90.0, 0.0);

    let instances = vec![
        Instance { model_matrix: model_matrix_from_position_and_rotation(model_position1, model_rotation1).into() },
        Instance { model_matrix: model_matrix_from_position_and_rotation(model_position2, model_rotation2).into() },
        Instance { model_matrix: model_matrix_from_position_and_rotation(model_position3, model_rotation3).into() },
        Instance { model_matrix: model_matrix_from_position_and_rotation(model_position4, model_rotation4).into() },
    ];
    let num_instances = instances.len() as u32;

    let instance_buffer_descriptor = wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instances),
        usage: wgpu::BufferUsage::VERTEX,
    };
    let instance_buffer = device.create_buffer_init(&instance_buffer_descriptor);
    let instance_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
        step_mode: wgpu::InputStepMode::Instance,
        attributes: &[
            // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
            // for each vec4. We'll have to reassemble the mat4 in
            // the shader.
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 5,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 6,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                shader_location: 7,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                shader_location: 8,
                format: wgpu::VertexFormat::Float32x4,
            },
        ],
    };

    let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            &uniform_bind_group_layout,
        ],
        push_constant_ranges: &[],
    };
    let render_pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

    let fragment_state = wgpu::FragmentState {
        module: &shader_module,
        entry_point: "main",
        targets: &[wgpu::ColorTargetState {
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
            entry_point: "main",
            buffers: &[
                vertex_buffer_layout,
                instance_buffer_layout,
            ],
        },
        fragment: Some(fragment_state),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    };
    let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    let size = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        depth_or_array_layers: 1,
    };
    let mut depth_texture_descriptor = wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT | wgpu::TextureUsage::SAMPLED,
    };
    let depth_texture = device.create_texture(&depth_texture_descriptor);

    let mut depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    event_loop.run(move |event, _, control_flow| {
        let mut update = || {
            if keyboard.is_key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
            }

            if keyboard.is_key_pressed(VirtualKeyCode::F1) {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            }

            if keyboard.is_key_pressed(VirtualKeyCode::F2) {
                window.set_fullscreen(None);
            }

            if keyboard.is_key_pressed(VirtualKeyCode::Key1) {
                println!("1");
            }
        };

        match event {
            Event::WindowEvent { ref event, window_id } => {
                if window_id != window.id() {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        keyboard.handle_input(input);
                    },
                    WindowEvent::Resized(physical_size) => {
                        let size = *physical_size;
                        swap_chain_descriptor.width = size.width;
                        swap_chain_descriptor.height = size.height;
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

                        depth_texture_descriptor.size.width = size.width;
                        depth_texture_descriptor.size.height = size.height;
                        let depth_texture = device.create_texture(&depth_texture_descriptor);
                        depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let size = **new_inner_size;
                        swap_chain_descriptor.width = size.width;
                        swap_chain_descriptor.height = size.height;
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

                        depth_texture_descriptor.size.width = size.width;
                        depth_texture_descriptor.size.height = size.height;
                        let depth_texture = device.create_texture(&depth_texture_descriptor);
                        depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
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
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: true,
                            }),
                            stencil_ops: None,
                        }),
                    };
                    let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..num_indices, 0, 0..num_instances);
                }

                let command_buffer = encoder.finish();
                queue.submit(std::iter::once(command_buffer));
            },
            Event::MainEventsCleared => {
                update();
                window.request_redraw();
            },
            _ => {},
        }
    });
}

fn model_matrix_from_position_and_rotation(position: cgmath::Vector3<f32>, rotation: cgmath::Vector3<f32>) -> cgmath::Matrix4<f32> {
    let position_matrix = cgmath::Matrix4::from_translation(position);
    let rotation_matrix = cgmath::Matrix4::from_angle_x(cgmath::Deg(rotation.x)) * cgmath::Matrix4::from_angle_y(cgmath::Deg(rotation.y)) * cgmath::Matrix4::from_angle_z(cgmath::Deg(rotation.z));

    position_matrix * rotation_matrix
}