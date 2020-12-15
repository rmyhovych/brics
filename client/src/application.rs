use crate::input::InputState;
use crate::renderer::Renderer;

use crate::scene::Scene;

use winit;

pub trait Application<T: 'static + Scene> {
    fn create_scene(&mut self, renderer: &mut Renderer) -> T;

    fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new();
        let mut renderer: Renderer = futures::executor::block_on(Renderer::new(&event_loop));
        let mut scene = self.create_scene(&mut renderer);
        scene.setup_logic(&mut renderer);

        let mut swap_chain = renderer.create_swap_chain();
        let mut input_state = InputState::new();
        let mut redraw_handler = RedrawHandler::new();

        event_loop.run(move |event, _, control_flow| {
            Self::suspend_control_flow(control_flow);

            match event {
                winit::event::Event::MainEventsCleared => {
                    redraw_handler.request(&renderer);
                }

                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {
                        input_state.handle(&event);
                    }
                },
                winit::event::Event::Suspended | winit::event::Event::Resumed => {
                    println!("EVENT [{:?}]", event);
                }
                winit::event::Event::RedrawRequested(_) => {
                    let frame = match swap_chain.get_current_frame() {
                        Ok(frame) => frame,
                        Err(_) => {
                            swap_chain = renderer.create_swap_chain();
                            swap_chain
                                .get_current_frame()
                                .expect("Failed to acquire next swap chain texture!")
                        }
                    };

                    scene.step(&input_state, &mut renderer);
                    renderer.render(&frame);
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

    fn request(&mut self, rend: &Renderer) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.previous.elapsed() > std::time::Duration::from_millis(16) {
                rend.request_redraw();
                self.previous = std::time::Instant::now();
            }
        }

        #[cfg(target_arch = "wasm32")]
        app.request_redraw();
    }
}
