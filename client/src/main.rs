use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::util::DeviceExt;

mod shader_compiler;
use shader_compiler::ShaderCompiler;

#[derive(Debug)]
struct Setup {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

struct MVP {
    projection: cgmath::Matrix4<f32>,
    view: cgmath::Matrix4<f32>,
    model: cgmath::Matrix4<f32>,
    color: cgmath::Vector3<f32>,
}

impl MVP {
    fn as_ref(&self) -> &[f32; 52] {
        unsafe { return mem::transmute(self) }
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        Vertex {
            pos: [-1.0, 1.0, 0.0],
        },
        Vertex {
            pos: [1.0, 1.0, 0.0],
        },
        Vertex {
            pos: [0.0, -1.0, 0.0],
        },
    ]
    .to_vec();

    let index_data = [0, 1, 2].to_vec();

    return (vertex_data, index_data);
}

fn create_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
    return winit::window::WindowBuilder::new()
        .with_title("rustgame")
        .with_inner_size(winit::dpi::Size::from(winit::dpi::PhysicalSize {
            width: 500,
            height: 500,
        }))
        .build(&event_loop)
        .unwrap();
}

async fn get_setup() -> Setup {
    let chrome_tracing_dir = std::env::var("WGPU_CHROME_TRACE");
    wgpu_subscriber::initialize_default_subscriber(
        chrome_tracing_dir.as_ref().map(std::path::Path::new).ok(),
    );

    let event_loop = winit::event_loop::EventLoop::new();
    let window = create_window(&event_loop);
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let surface = unsafe { instance.create_surface(&window) };

    let adapter: wgpu::Adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let optional_features = wgpu::Features::empty();
    let required_features = wgpu::Features::empty();
    let adapter_features = adapter.features();
    assert!(
        adapter_features.contains(required_features),
        "Adapter does not support required features for this example: {:?}",
        required_features - adapter_features
    );

    let needed_limits = wgpu::Limits::default();

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: (optional_features & adapter_features) | required_features,
                limits: needed_limits,
                shader_validation: true,
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .unwrap();
    return Setup {
        window,
        event_loop,
        instance,
        surface,
        adapter,
        device,
        queue,
    };
}

fn run(setup: Setup) {
    let (mut pool, spawner) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();
        (local_pool, spawner)
    };

    let window_size: winit::dpi::PhysicalSize<u32> = setup.window.inner_size();
    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = setup
        .device
        .create_swap_chain(&setup.surface, &swap_chain_descriptor);
    println!("Created swapchain : {:?}", swap_chain);

    let vertex_size = mem::size_of::<Vertex>();
    let (vertex_data, index_data) = create_vertices();

    let vertex_buffer = setup
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });
    println!("Vertex Buffer : {:?}", vertex_buffer);

    let index_buffer = setup
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });
    println!("Index Buffer : {:?}", index_buffer);

    let mat_size: u64 = mem::size_of::<MVP>() as u64;
    println!("Mat : {}", mat_size);
    let bind_group_layout =
        setup
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(mat_size),
                    },
                    count: None,
                }],
            });

    let pipeline_layout = setup
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let uniforms = MVP {
        projection: cgmath::perspective(cgmath::Deg(90.0), 1.0, 0.01, 1000.0),
        view: cgmath::Matrix4::look_at(
            cgmath::Point3::new(1.5, -5.0, 3.0),
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_z(),
        ),
        model: cgmath::Matrix4::from_scale(1.0 as f32),
        color: cgmath::Vector3 {
            x: 0.2,
            y: 0.8,
            z: 0.2,
        },
    };

    let uniform_buffer = setup
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(uniforms.as_ref()),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

    let bind_group = setup.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
        }],
    });

    let mut sc = ShaderCompiler::new();
    let vertex_spiv = sc.compile_vertex("shaders/shader.vert");
    let fragment_spiv = sc.compile_fragment("shaders/shader.frag");
    let vertex_module = setup
        .device
        .create_shader_module(vertex_spiv.get_module_source());
    let fragment_module = setup
        .device
        .create_shader_module(fragment_spiv.get_module_source());

    let pipeline = setup
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fragment_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: swap_chain_descriptor.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: vertex_size as u64,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float3,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            sample_count: 1,
            sample_mask: 0,
            alpha_to_coverage_enabled: false,
        });
    println!("Created Pipeline : {:?}", pipeline);

    let mut last_update_inst = std::time::Instant::now();

    println!("Entering render loop...");

    let instance = setup.instance;
    let adapter = setup.adapter;
    let event_loop = setup.event_loop;
    let window = setup.window;
    let surface = setup.surface;
    let device = setup.device;
    let queue = setup.queue;

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &swap_chain); // force ownership by the closure
        *control_flow = if cfg!(feature = "metal_auto_captyre") {
            winit::event_loop::ControlFlow::Exit
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
                winit::event_loop::ControlFlow::WaitUntil(
                    std::time::Instant::now() + std::time::Duration::from_millis(10),
                )
            }
            #[cfg(target_arch = "wasm32")]
            {
                winit::event_loop::ControlFlow::Poll
            }
        };

        match event {
            winit::event::Event::MainEventsCleared => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if last_update_inst.elapsed() > std::time::Duration::from_millis(20) {
                        window.request_redraw();
                        last_update_inst = std::time::Instant::now();
                    }

                    pool.run_until_stalled();
                }

                #[cfg(target_arch = "wasm32")]
                window.request_redraw();
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            winit::event::Event::RedrawRequested(_) => {
                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => {
                        swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
                        swap_chain
                            .get_current_frame()
                            .expect("Failed to acquire next swap chain texture!")
                    }
                };

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.output.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.push_debug_group("Prepare data for draw.");
                    rpass.set_pipeline(&pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_index_buffer(index_buffer.slice(..));
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.pop_debug_group();
                    rpass.insert_debug_marker("Draw!");
                    rpass.draw_indexed(0..index_data.len() as u32, 0, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            }
            _ => {}
        }
    });
}

fn main() {
    println!("Hello, world!");
    let setup = futures::executor::block_on(get_setup());
    run(setup);
}
