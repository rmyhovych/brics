extern crate brics;

mod application;
mod script;
mod vertex;
mod visual;

use application::BasicApplication;
use brics::run::run;

fn main() {
    run::<BasicApplication>(60);
}
