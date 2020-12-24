use winit::dpi::PhysicalPosition;
use winit::event::{
    ElementState, KeyboardInput, MouseButton, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
};

pub struct InputState {
    pub mouse: MouseState,
    pub keyboard: KeyboardState,

    pub touch: TouchState,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse: MouseState::new(),
            keyboard: KeyboardState::new(),
            touch: TouchState::new(),
        }
    }

    pub fn handle(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse.handle_input(*button, *state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse.handle_movement(position);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                self.keyboard.handle(input);
            }
            WindowEvent::Touch(touch) => {
                self.touch.handle(touch);
            }
            _ => (),
        };
    }
}

/* ---------------------------- MOUSE ---------------------------- */
pub struct MouseState {
    pub location: PhysicalPosition<f64>,
    pub button: Option<MouseButton>,
}

impl MouseState {
    fn new() -> Self {
        Self {
            location: PhysicalPosition { x: 0.0, y: 0.0 },
            button: None,
        }
    }

    fn handle_input(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => self.button = Some(button),
            ElementState::Released => self.button = None,
        };
    }

    fn handle_movement(&mut self, location: &PhysicalPosition<f64>) {
        self.location = *location;
    }
}

/* ---------------------------- KEYBOARD ---------------------------- */
pub struct KeyboardState {
    pub pressed: std::collections::HashSet<VirtualKeyCode>,
}

impl KeyboardState {
    fn new() -> Self {
        Self {
            pressed: std::collections::HashSet::new(),
        }
    }

    fn handle(&mut self, keyboard_input: &KeyboardInput) {
        if let Some(key) = keyboard_input.virtual_keycode {
            match keyboard_input.state {
                ElementState::Pressed => {
                    self.pressed.insert(key);
                }
                ElementState::Released => {
                    if self.pressed.contains(&key) {
                        self.pressed.remove(&key);
                    }
                }
            }
        }
    }
}

/* ---------------------------- TOUCH ---------------------------- */
pub struct TouchState {
    pub fingers: std::collections::HashMap<u64, Finger>,
}

impl TouchState {
    fn new() -> Self {
        Self {
            fingers: std::collections::HashMap::new(),
        }
    }

    fn handle(&mut self, touch: &Touch) {
        match touch.phase {
            TouchPhase::Started => {
                self.fingers.insert(
                    touch.id,
                    Finger {
                        id: touch.id,
                        location: touch.location,
                    },
                );
            }
            TouchPhase::Cancelled | TouchPhase::Ended => {
                self.fingers.remove(&touch.id);
            }
            TouchPhase::Moved => match self.fingers.get_mut(&touch.id) {
                Some(finger) => finger.location = touch.location,
                None => (),
            },
        };
    }
}

pub struct Finger {
    pub id: u64,
    pub location: PhysicalPosition<f64>,
}
