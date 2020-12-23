use crate::logic::GameLogic;
use crate::renderer::Renderer;

use winit;

pub struct Application<R: 'static + Renderer, L: 'static + GameLogic<R>> {
    _phantom_renderer: std::marker::PhantomData<R>,
    _phantom_game_logic: std::marker::PhantomData<L>,
}

impl<R: 'static + Renderer, L: 'static + GameLogic<R>> Application<R, L> {
    pub fn new() -> Self {
        Self {
            _phantom_renderer: std::marker::PhantomData,
            _phantom_game_logic: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new();

        let mut renderer = R::new(&event_loop);
        let mut game_logic = L::new();
        game_logic.setup(&mut renderer);

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
                        game_logic.handle_input(&event);
                    }
                },
                winit::event::Event::Suspended | winit::event::Event::Resumed => {
                    println!("EVENT [{:?}]", event);
                }
                winit::event::Event::RedrawRequested(_) => {
                    renderer.render();
                    game_logic.step();
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

    fn request(&mut self, rend: &dyn Renderer) {
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
