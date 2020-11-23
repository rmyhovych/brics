use cgmath;
use wgpu;

mod layout;
mod pipeline;

mod binding;

mod camera;

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

struct Vertex {
    position: cgmath::Point3<f32>,
    normal: cgmath::Vector3<f32>,
}

impl std::clone::Clone for Vertex {
    fn clone(&self) -> Self {
        *self
    }
}

impl std::marker::Copy for Vertex {}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
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
    .iter()
    .map(|raw_vertex| Vertex {
        position: cgmath::Point3 {
            x: raw_vertex[0],
            y: raw_vertex[1],
            z: raw_vertex[2],
        },
        normal: cgmath::Vector3 {
            x: raw_vertex[3],
            y: raw_vertex[4],
            z: raw_vertex[5],
        },
    })
    .collect::<Vec<Vertex>>();
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

        #[cfg(not(target_os = "android"))]
        format: wgpu::TextureFormat::Bgra8Unorm,

        #[cfg(target_os = "android")]
        format: wgpu::TextureFormat::Rgba8Unorm,

        width: window_size.width,
        height: window_size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = setup
        .device
        .create_swap_chain(&setup.surface, &swap_chain_descriptor);
    println!("Created swapchain : {:?}", swap_chain);

    let camera = camera::Camera::look_at(
        &setup.device,
        &cgmath::Point3 {
            x: 5.0,
            y: 5.0,
            z: -5.5,
        },
        &cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        window_size.width as f32 / window_size.height as f32,
    );

    let object_layout: layout::Layout<Vertex> = layout::LayoutBuilder::new()
        .add_binding_layout(camera.get_binding_layout())
        .push_attribute_format(wgpu::VertexFormat::Float3)
        .push_attribute_format(wgpu::VertexFormat::Float3)
        .build::<Vertex>(&setup.device);

    let vertex_module = setup
        .device
        .create_shader_module(wgpu::include_spirv!("shaders/shader.vert.spv"));
    let fragment_module = setup
        .device
        .create_shader_module(wgpu::include_spirv!("shaders/shader.frag.spv"));

    let pipeline = pipeline::Builder::new()
        .set_shaders(&vertex_module, &fragment_module)
        .set_color_format(swap_chain_descriptor.format)
        .set_object_layout(&object_layout)
        .build(&setup.device);

    println!("Created Pipeline : [{:?}]", pipeline);

    let (vertex_data, index_data) = create_vertices();
    let entity = object_layout.create_entity(
        &setup.device,
        &vertex_data,
        &index_data,
        &vec![camera.get_binding()],
    );
    println!("Created Entity : [{:?}]", entity);

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

    let window = setup.window;
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(16));

        #[cfg(target_os = "android")]
        request_redraw();

        #[cfg(not(target_os = "android"))]
        window.request_redraw();
    });

    let instance = setup.instance;
    let adapter = setup.adapter;
    let event_loop = setup.event_loop;
    let surface = setup.surface;
    let device = setup.device;
    let queue = setup.queue;

    println!("Entering render loop...");
    let mut finger_position = winit::dpi::PhysicalPosition { x: 0.0, y: 0.0 };
    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &swap_chain); // force ownership by the closure

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
                    &camera,
                    &entity,
                    &depth_texture_view,
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
    pipeline: &pipeline::Pipeline,
    camera: &camera::Camera,
    entity: &layout::Entity,
    depth_texture_view: &wgpu::TextureView,
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
        pipeline.apply_on_render_pass(&mut rpass, queue);
        camera.apply_on_renderpass(&mut rpass, queue);
        entity.apply_on_render_pass(&mut rpass, queue);
    }

    queue.submit(Some(encoder.finish()));
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    #[cfg(target_os = "android")]
    {
        wait_for_window();
    }

    println!("Hello, world!");
    let setup = futures::executor::block_on(get_setup());
    run(setup);
}

#[cfg(target_os = "android")]
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

#[cfg(target_os = "android")]
fn wait_for_window() {
    loop {
        let native_window = match ndk_glue::native_window().as_ref() {
            Some(_) => break,
            None => continue,
        };
    }
}
