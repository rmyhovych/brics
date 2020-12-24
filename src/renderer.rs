use crate::render_pass::RenderPass;

use wgpu;

pub struct Renderer {
    render_passes: Vec<RenderPass>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            render_passes: Vec::new(),
        }
    }

    pub fn add_render_pass(&mut self, rpass: RenderPass) -> u32 {
        self.render_passes.push(rpass);

        (self.render_passes.len() - 1) as u32
    }

    pub fn get_render_pass(&mut self, id: u32) -> &mut RenderPass {
        &mut self.render_passes[id as usize]
    }

    pub fn submit(&self, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainFrame) {
        self.render_passes
            .iter()
            .for_each(|rpass| rpass.submit(encoder, frame));
    }
}
