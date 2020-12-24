extern crate rustgame;

mod logic;
mod visual;
mod vertex;

use rustgame::application::Application;

fn main() {
    let mut app: Application<visual::MainVisual, logic::MainLogic> = Application::new();
    app.run();
}
