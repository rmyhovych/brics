extern crate brics;

mod logic;
mod vertex;
mod visual;

use brics::application::Application;

fn main() {
    let mut app: Application<visual::MainVisual, logic::MainLogic> = Application::new();
    app.run();
}
