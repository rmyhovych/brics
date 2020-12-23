extern crate rustgame;

mod logic;
mod renderer;
mod vertex;

use rustgame::application::Application;

fn main() {
    let mut app: Application<renderer::MainRenderer, logic::MainLogic> = Application::new();
    app.run();
}
