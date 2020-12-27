use super::application::{Application, Visual};

pub fn run<A: Application + 'static>(fps: u32) {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut app = A::new(&event_loop);

    let mut redraw_handler = RedrawHandler::new();
    event_loop.run(move |event, _, control_flow| {
        let _ = &app;

        suspend_control_flow(control_flow);

        match event {
            winit::event::Event::MainEventsCleared => {
                redraw_handler.request(&app);
            }

            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => {
                    app.handle_input(&event);
                }
            },
            winit::event::Event::Suspended | winit::event::Event::Resumed => {
                println!("EVENT [{:?}]", event);
            }
            winit::event::Event::RedrawRequested(_) => {
                app.step();
            }
            _ => {}
        }
    });
}

fn suspend_control_flow(control_flow: &mut winit::event_loop::ControlFlow) {
    *control_flow = {
        #[cfg(not(target_arch = "wasm32"))]
        {
            winit::event_loop::ControlFlow::WaitUntil(
                std::time::Instant::now() + std::time::Duration::from_millis(15),
            )
        }
        #[cfg(target_arch = "wasm32")]
        {
            ControlFlow::Poll
        }
    };
}

/*--------------------------------------------------------------------------------------------------*/

struct RedrawHandler {
    #[cfg(not(target_arch = "wasm32"))]
    previous: std::time::Instant,
}

impl RedrawHandler {
    fn new() -> Self {
        Self {
            previous: std::time::Instant::now(),
        }
    }

    fn request(&mut self, app: &dyn Application) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.previous.elapsed() > std::time::Duration::from_millis(16) {
                app.request_redraw();
                self.previous = std::time::Instant::now();
            }
        }

        #[cfg(target_arch = "wasm32")]
        app.request_redraw();
    }
}
