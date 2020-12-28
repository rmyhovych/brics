use super::application::{Application, ApplicationController};

pub fn run<A: Application + 'static, C: ApplicationController<A> + 'static>(fps: u32) {
    let event_loop = winit::event_loop::EventLoop::new();

    let mut app = Box::new(A::new(&event_loop));
    let mut controller = C::new(&mut app);

    let mut redraw_handler = RedrawHandler::new(fps);
    event_loop.run(move |event, _, control_flow| {
        let _ = &app;

        suspend_control_flow(control_flow);

        match event {
            winit::event::Event::MainEventsCleared => {
                redraw_handler.request(app.as_ref());
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
                controller.step(app.as_mut());
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

    wait_ms: u64,
}

impl RedrawHandler {
    fn new(fps: u32) -> Self {
        Self {
            previous: std::time::Instant::now(),
            wait_ms: (1000 / (fps + 2)) as u64,
        }
    }

    fn request(&mut self, app: &dyn Application) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.previous.elapsed() > std::time::Duration::from_millis(self.wait_ms) {
                app.request_redraw();
                self.previous = std::time::Instant::now();
            }
        }

        #[cfg(target_arch = "wasm32")]
        app.request_redraw();
    }
}
