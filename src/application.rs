use winit;

pub trait GameLogic<R: Visual> {
    fn new() -> Self;

    fn setup(&mut self, renderer: &mut R);

    fn step(&mut self);

    fn handle_input(&mut self, event: &winit::event::WindowEvent);
}

pub trait Visual {
    fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self
    where
        Self: Sized;

    fn render(&mut self);

    fn request_redraw(&self);
}

pub struct Application<R: 'static + Visual, L: 'static + GameLogic<R>> {
    _phantom_renderer: std::marker::PhantomData<R>,
    _phantom_game_logic: std::marker::PhantomData<L>,
}

impl<R: 'static + Visual, L: 'static + GameLogic<R>> Application<R, L> {
    pub fn new() -> Self {
        Self {
            _phantom_renderer: std::marker::PhantomData,
            _phantom_game_logic: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) {
        let event_loop = winit::event_loop::EventLoop::new();

        let mut visual = Box::new(R::new(&event_loop));
        let mut game_logic = Box::new(L::new());
        game_logic.setup(visual.as_mut());

        let mut redraw_handler = RedrawHandler::new();
        event_loop.run(move |event, _, control_flow| {
            let _ = (&visual, &game_logic);
            Self::suspend_control_flow(control_flow);

            match event {
                winit::event::Event::MainEventsCleared => {
                    redraw_handler.request(visual.as_ref());
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
                    visual.render();
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

    fn request(&mut self, rend: &dyn Visual) {
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
