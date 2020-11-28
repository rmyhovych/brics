use wgpu;
use winit;

use crate::input;
use crate::pipeline;
use crate::resource;
use crate::shader;

use shaderc;

pub struct Application {
    instance: wgpu::Instance,
    pub device: wgpu::Device,
    queue: wgpu::Queue,

    window: winit::window::Window,
    surface: wgpu::Surface,

    depth_texture_view: wgpu::TextureView,

    swap_chain_format: wgpu::TextureFormat,

    pipelines: Vec<pipeline::Pipeline>,
}

impl Application {
    pub async fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Application {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let window = winit::window::WindowBuilder::new()
            .with_title("rustgame")
            .build(event_loop)
            .unwrap();

        let surface = unsafe { instance.create_surface(&window) };

        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let needed_limits = wgpu::Limits::default();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: Self::get_features(&adapter),
                    limits: needed_limits,
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let depth_texture_view = Self::create_depth_texture_view(&device, &window.inner_size());
        Self {
            instance,
            device,
            queue,

            window,
            surface,

            depth_texture_view,

            #[cfg(not(target_os = "android"))]
            swap_chain_format: wgpu::TextureFormat::Bgra8Unorm,
            #[cfg(target_os = "android")]
            swap_chain_format: wgpu::TextureFormat::Rgba8Unorm,

            pipelines: Vec::new(),
        }
    }

    pub fn step<F>(&self, game_loop: &mut F, input_state: &input::InputState) -> ()
    where
        F: FnMut(&wgpu::Queue, &input::InputState),
    {
        game_loop(&self.queue, input_state);
    }

    pub fn render(&self, frame: &wgpu::SwapChainFrame) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
                    attachment: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            for p in self.pipelines.iter() {
                p.render(&mut rpass)
            }
        }
        self.queue.submit(Some(encoder.finish()));
    }

    pub fn create_swap_chain(&self) -> wgpu::SwapChain {
        let window_size = self.window.inner_size();
        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: self.swap_chain_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        self.device
            .create_swap_chain(&self.surface, &swap_chain_descriptor)
    }

    pub fn request_redraw(&self) {
        #[cfg(target_os = "android")]
        {
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

        #[cfg(not(target_os = "android"))]
        self.window.request_redraw();
    }

    pub fn get_window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn create_pipeline<T: pipeline::Vertex>(
        &mut self,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
        binding_entries: pipeline::BindingEntries,

        entity_descriptors: &Vec<pipeline::EntityDescriptor<T>>,
    ) {
        let mut shader_compiler = shader::ShaderCompiler::new();
        let shaders = pipeline::Shaders {
            vertex_module: self.device.create_shader_module(
                shader_compiler.compile(vertex_shader_path, shaderc::ShaderKind::Vertex),
            ),
            fragment_module: self.device.create_shader_module(
                shader_compiler.compile(fragment_shader_path, shaderc::ShaderKind::Fragment),
            ),
        };

        self.pipelines.push(pipeline::Pipeline::new::<T>(
            &self.device,
            &shaders,
            &binding_entries,
            self.swap_chain_format,
        ));

        let pipeline = self.pipelines.last_mut().unwrap();
        for desc in entity_descriptors {
            pipeline.add_entity(&self.device, &desc);
        }
    }

    /*-------------------------------------------------*/

    fn get_features(adapter: &wgpu::Adapter) -> wgpu::Features {
        let optional_features = wgpu::Features::empty();
        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        assert!(
            adapter_features.contains(required_features),
            "Adapter does not support required features for this example: {:?}",
            required_features - adapter_features
        );

        (optional_features & adapter_features) | required_features
    }

    fn create_depth_texture_view(
        device: &wgpu::Device,
        window_size: &winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
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

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
