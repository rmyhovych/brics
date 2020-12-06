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
    color_attachment: Option<Attachment<wgpu::Color>>,
    depth_attachment: Option<Attachment<f32>>,

    pipelines: Vec<Pipeline>,
}

impl RenderPass {
    pub fn new() -> Self {
        Self {
            color_attachment: None,
            depth_attachment: None,

            pipelines: Vec::new(),
        }
    }

    pub fn set_color_attachment(
        &mut self,
        color_view: AttachmentView,
        color_ops: wgpu::Operations<wgpu::Color>,
    ) -> &mut Self {
        self.color_attachment = Some(Attachment {
            view: color_view,
            ops: color_ops,
        });

        self
    }

    pub fn set_depth_attachment(
        &mut self,
        depth_view: wgpu::TextureView,
        depth_ops: wgpu::Operations<f32>,
    ) -> &mut Self {
        self.depth_attachment = Some(Attachment {
            view: AttachmentView::Static(depth_view),
            ops: depth_ops,
        });

        self
    }

    pub fn add_pipeline(&mut self, pipeline: Pipeline) -> &mut Pipeline {
        self.pipelines.push(pipeline);

        self.pipelines.last_mut().unwrap()
    }

    pub fn submit(&self, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainFrame) {
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

        let mut rpass = match &self.color_attachment {
            Some(color_attachment) => {
                let color_attachment_view: &wgpu::TextureView = match &color_attachment.view {
                    AttachmentView::Dynamic => &frame.output.view,
                    AttachmentView::Static(view) => view,
                };
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &color_attachment_view,
                        resolve_target: None,
                        ops: color_attachment.ops,
                    }],
                    depth_stencil_attachment: depth_attachment_descriptor,
                })
            }
            None => encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[],
                depth_stencil_attachment: depth_attachment_descriptor,
            }),
        };

        for p in self.pipelines.iter() {
            p.render(&mut rpass)
        }
    }
}
