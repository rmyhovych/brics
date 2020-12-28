extern crate brics;

mod application;
mod controller;
mod vertex;
mod visual;

use application::BasicApplication;
use controller::BasicController;

use brics::run::run;

fn main() {
    run::<BasicApplication, BasicController>(60);
}
