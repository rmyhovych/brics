use crate::pipeline::Pipeline;
use wgpu;

pub enum AttachmentView {
    Dynamic,
    Static(wgpu::TextureView),
}

struct Attachment<T> {
    view: AttachmentView,
    ops: wgpu::Operations<T>,
}

pub struct RenderPass {
    color_attachment: Attachment<wgpu::Color>,
    depth_attachment: Option<Attachment<f32>>,
}

impl RenderPass {
    pub fn new(color_view: AttachmentView, color_ops: wgpu::Operations<wgpu::Color>) -> Self {
        Self {
            color_attachment: Attachment {
                view: color_view,
                ops: color_ops,
            },

            depth_attachment: None,
        }
    }

    pub fn add_depth_attachment(
        &mut self,
        depth_view: wgpu::TextureView,
        depth_ops: wgpu::Operations<f32>,
    ) {
        self.depth_attachment = Some(Attachment {
            view: AttachmentView::Static(depth_view),
            ops: depth_ops,
        });
    }

    pub fn submit(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pipelines: &Vec<Pipeline>,
        frame: &wgpu::SwapChainFrame,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let color_attachment_view: &wgpu::TextureView = match &self.color_attachment.view {
                AttachmentView::Dynamic => &frame.output.view,
                AttachmentView::Static(view) => view,
            };

            let depth_attachment_descriptor = match &self.depth_attachment {
                None => None,
                Some(attachment) => match &attachment.view {
                    AttachmentView::Dynamic => None,
                    AttachmentView::Static(depth_view) => {
                        Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &depth_view,
                            depth_ops: Some(attachment.ops),
                            stencil_ops: None,
                        })
                    }
                },
            };

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &color_attachment_view,
                    resolve_target: None,
                    ops: self.color_attachment.ops,
                }],
                depth_stencil_attachment: depth_attachment_descriptor,
            });

            for p in pipelines.iter() {
                p.render(&mut rpass)
            }
        }

        queue.submit(Some(encoder.finish()));
    }
}
