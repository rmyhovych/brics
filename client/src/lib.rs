use cgmath;
use wgpu;

use rand::RngCore;

mod object;
use object::{Object, ObjectFamily};

mod layout;
use layout::LayoutHandler;

mod camera;
use camera::Camera;

mod pipeline;
mod object_factory;

mod binding;


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

fn create_vertices() -> (Vec<[f32; 6]>, Vec<u16>) {
    let vertex_data = [
        // Back face
        [-1.0, -1.0, 1.0, 0.0, 0.0, 1.0],
        [1.0, -1.0, 1.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        [-1.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        // Front face
        [-1.0, -1.0, -1.0, 0.0, 0.0, -1.0],
        [-1.0, 1.0, -1.0, 0.0, 0.0, -1.0],
        [1.0, 1.0, -1.0, 0.0, 0.0, -1.0],
        [1.0, -1.0, -1.0, 0.0, 0.0, -1.0],
        // Bottom face
        [-1.0, 1.0, -1.0, 0.0, 1.0, 0.0],
        [-1.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0, 0.0, 1.0, 0.0],
        [1.0, 1.0, -1.0, 0.0, 1.0, 0.0],
        // Top face
        [-1.0, -1.0, -1.0, 0.0, -1.0, 0.0],
        [1.0, -1.0, -1.0, 0.0, -1.0, 0.0],
        [1.0, -1.0, 1.0, 0.0, -1.0, 0.0],
        [-1.0, -1.0, 1.0, 0.0, -1.0, 0.0],
        // Right face
        [1.0, -1.0, -1.0, 1.0, 0.0, 0.0],
        [1.0, 1.0, -1.0, 1.0, 0.0, 0.0],
        [1.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        [1.0, -1.0, 1.0, 1.0, 0.0, 0.0],
        // Left face
        [-1.0, -1.0, -1.0, -1.0, 0.0, 0.0],
        [-1.0, -1.0, 1.0, -1.0, 0.0, 0.0],
        [-1.0, 1.0, 1.0, -1.0, 0.0, 0.0],
        [-1.0, 1.0, -1.0, -1.0, 0.0, 0.0],
    ]
    .to_vec();
    let index_data = [
        0, 1, 3, 3, 1, 2, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ]
    .to_vec();

    return (vertex_data, index_data);
}

fn create_window(event_loop: &winit::event_loop::EventLoop<()>) -> winit::window::Window {
    return winit::window::WindowBuilder::new()
        .with_title("rustgame")
        .build(&event_loop)
        .unwrap();
}

fn random_number(rng: &mut rand::rngs::ThreadRng) -> f32 {
    (rng.next_u32() % 501) as f32 - 250.0
}

fn set_random_position(object: &mut Object) {
    let mut rng = rand::thread_rng();
    object.translate(
        random_number(&mut rng),
        random_number(&mut rng),
        random_number(&mut rng),
    );
    object.rotate(
        &cgmath::Vector3 {
            x: random_number(&mut rng),
            y: random_number(&mut rng),
            z: random_number(&mut rng),
        },
        &cgmath::Rad(random_number(&mut rng)),
    )
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
    let (mut pool, _) = {
        let local_pool = futures::executor::LocalPool::new();
        let spawner = local_pool.spawner();
        (local_pool, spawner)
    };

    let window_size: winit::dpi::PhysicalSize<u32> = setup.window.inner_size();
    println!("WINDOW SIZE {:?}", window_size);
    let swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = setup
        .device
        .create_swap_chain(&setup.surface, &swap_chain_descriptor);
    println!("Created swapchain : {:?}", swap_chain);

    let (vertex_data, index_data) = create_vertices();

    let mut object_family = ObjectFamily::new(&setup.device, &vertex_data, &index_data, 1000);
    object_family.get(0).set_scale(0.1, 0.1, 0.1);
    for i in 1..1000 {
        set_random_position(object_family.get(i));
    }

    let mut camera = Camera::look_at(
        &cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: -50.5,
        },
        &cgmath::Point3 {
            x: 0.3,
            y: 0.5,
            z: 1.0,
        },
        window_size.width as f32 / window_size.height as f32,
        &setup.device,
    );

    let bind_group_layout =
        setup
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    object_family.get_bind_group_layout_entry(),
                    camera.get_bind_group_layout_entry(),
                ],
            });

    let pipeline_layout = setup
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let bind_group = setup.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            object_family.get_bind_group_entry(),
            camera.get_bind_group_entry(),
        ],
    });

    let vertex_module = setup
        .device
        .create_shader_module(wgpu::include_spirv!("shaders/shader.vert.spirv"));
    let fragment_module = setup
        .device
        .create_shader_module(wgpu::include_spirv!("shaders/shader.frag.spirv"));

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
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: swap_chain_descriptor.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: std::mem::size_of::<[f32; 6]>() as u64, // TODO: Separate trait
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float3,
                            offset: 12,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            sample_count: 1,
            sample_mask: 0,
            alpha_to_coverage_enabled: false,
        });
    println!("Created Pipeline : {:?}", pipeline);

    let depth_texture = setup.device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: window_size.width,
            height: window_size.height,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        label: None,
    });

    let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut last_update_inst = std::time::Instant::now();

    println!("Entering render loop...");

    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_millis(16));
        request_redraw();
    });

    let instance = setup.instance;
    let adapter = setup.adapter;
    let event_loop = setup.event_loop;
    let window = setup.window;
    let surface = setup.surface;
    let device = setup.device;
    let queue = setup.queue;

    let multiplier = 0.003;
    let mut finger_position = winit::dpi::PhysicalPosition { x: 0.0, y: 0.0 };
    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &swap_chain, &object_family); // force ownership by the closure

        *control_flow = if cfg!(feature = "metal-auto-capture") {
            winit::event_loop::ControlFlow::Exit
        } else {
            winit::event_loop::ControlFlow::WaitUntil(
                std::time::Instant::now() + std::time::Duration::from_millis(10),
            )
        };

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::Touch(touch) => match touch.phase {
                    winit::event::TouchPhase::Started => finger_position = touch.location,
                    winit::event::TouchPhase::Moved => {
                        let new_position = touch.location;
                        let dx = new_position.x - finger_position.x;
                        let dy = new_position.y - finger_position.y;
                        camera
                            .rotate_around_center(-multiplier * dx as f32, -multiplier * dy as f32);

                        finger_position = new_position
                    }
                    _ => (),
                },
                _ => {}
            },
            winit::event::Event::Suspended | winit::event::Event::Resumed => {
                println!("EVENT [{:?}]", event);
            }
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

                render(
                    &frame,
                    &device,
                    &pipeline,
                    &depth_texture_view,
                    &bind_group,
                    &camera,
                    &object_family,
                    &queue,
                );
            }
            _ => {}
        }
    });
}

fn render(
    frame: &wgpu::SwapChainFrame,
    device: &wgpu::Device,
    pipeline: &wgpu::RenderPipeline,
    depth_texture_view: &wgpu::TextureView,
    bind_group: &wgpu::BindGroup,
    camera: &Camera,
    object_family: &ObjectFamily,
    queue: &wgpu::Queue,
) {
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: false,
                }),
                stencil_ops: None,
            }),
        });
        rpass.set_pipeline(&pipeline);
        rpass.set_bind_group(0, &bind_group, &[]);
        camera.apply_on_renderpass(&mut rpass, queue);
        object_family.apply_on_renderpass(&mut rpass, queue);
    }

    queue.submit(Some(encoder.finish()));
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    wait_for_window();
    println!("Hello, world!");
    let setup = futures::executor::block_on(get_setup());
    run(setup);
}

pub fn request_redraw() {
    match ndk_glue::native_window().as_ref() {
        Some(native_window) => {
            let a_native_window: *mut ndk_sys::ANativeWindow = native_window.ptr().as_ptr();
            let a_native_activity: *mut ndk_sys::ANativeActivity =
                ndk_glue::native_activity().ptr().as_ptr();
            unsafe {
                match (*(*a_native_activity).callbacks).onNativeWindowRedrawNeeded {
                    Some(callback) => callback(a_native_activity, a_native_window),
                    None => (),
                };
            };
        }
        None => (),
    }
}

fn wait_for_window() {
    loop {
        let native_window = match ndk_glue::native_window().as_ref() {
            Some(_) => break,
            None => continue,
        };
    }
}
