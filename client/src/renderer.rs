use wgpu::{self, util::DeviceExt};
use winit;

use crate::{binding, handle, pipeline, render_pass, shader};

use shaderc;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    window: winit::window::Window,
    surface: wgpu::Surface,

    render_passes: Vec<render_pass::RenderPass>,
}

impl Renderer {
    pub async fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let window = winit::window::WindowBuilder::new()
            .with_title("rustgame")
            .with_inner_size(winit::dpi::PhysicalSize {
                width: 1920,
                height: 1080,
            })
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

        Self {
            device,
            queue,

            window,
            surface,

            render_passes: Vec::new(),
        }
    }

    pub fn get_swapchain_color_format() -> wgpu::TextureFormat {
        #[cfg(not(target_os = "android"))]
        let swapchain_color_format = wgpu::TextureFormat::Bgra8Unorm;

        #[cfg(target_os = "android")]
        let swapchain_color_format = wgpu::TextureFormat::Rgba8Unorm;

        swapchain_color_format
    }

    pub fn render(&self, frame: &wgpu::SwapChainFrame) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.render_passes
            .iter()
            .for_each(|rpass| rpass.submit(&mut encoder, frame));

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn create_swap_chain(&self) -> wgpu::SwapChain {
        let window_size = self.window.inner_size();
        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: Self::get_swapchain_color_format(),
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

    pub fn update_binding<B: binding::Binding>(
        &self,
        binding_handle: &impl handle::BindingHandle<B>,
    ) {
        binding_handle.update(&self.queue);
    }

    pub fn create_binding<B: binding::Binding>(
        &self,
        binding_layout: &impl binding::BindingLayout<B>,
    ) -> B {
        binding_layout.create_binding(&self.device)
    }

    pub fn add_render_pass(&mut self, rpass: render_pass::RenderPass) {
        self.render_passes.push(rpass);
    }

    pub fn create_geometry<T: pipeline::Vertex>(
        &self,
        vertices: Vec<T>,
        indices: Vec<u16>,
    ) -> pipeline::Geometry {
        let vertex_buffer = self.create_device_buffer(&vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer = self.create_device_buffer(&indices, wgpu::BufferUsage::INDEX);

        let n_indices = indices.len() as u32;

        pipeline::Geometry::new(vertex_buffer, index_buffer, n_indices)
    }

    pub fn add_pipeline_entity(
        &self,
        pipeline: &mut pipeline::Pipeline,
        geometry: &pipeline::Geometry,
        bindings: Vec<&dyn binding::Binding>,
        n_instances: u32,
    ) {
        pipeline.add_entity(&self.device, geometry, bindings, n_instances);
    }

    pub fn create_pipeline<T: pipeline::Vertex>(
        &self,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
        binding_entries: pipeline::BindingEntries,

        color_state: Option<wgpu::ColorStateDescriptor>,
        depth_stencil_state: Option<wgpu::DepthStencilStateDescriptor>,
        rasterization_state: Option<wgpu::RasterizationStateDescriptor>,
    ) -> pipeline::Pipeline {
        let mut shader_compiler = shader::ShaderCompiler::new();
        let shaders = pipeline::Shaders {
            vertex_module: self.device.create_shader_module(
                shader_compiler.compile(vertex_shader_path, shaderc::ShaderKind::Vertex),
            ),
            fragment_module: self.device.create_shader_module(
                shader_compiler.compile(fragment_shader_path, shaderc::ShaderKind::Fragment),
            ),
        };

        pipeline::Pipeline::new::<T>(
            &self.device,
            &shaders,
            &binding_entries,
            color_state,
            depth_stencil_state,
            rasterization_state,
        )
    }

    pub fn create_depth_texture_view(&self) -> wgpu::TextureView {
        let window_size = self.window.inner_size();
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
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

    fn create_device_buffer<K>(&self, contents: &Vec<K>, usage: wgpu::BufferUsage) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: unsafe {
                    std::slice::from_raw_parts(
                        contents.as_slice().as_ptr() as *const u8,
                        std::mem::size_of::<K>() * contents.len(),
                    )
                },
                usage,
            })
    }
}
