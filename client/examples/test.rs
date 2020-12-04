extern crate rustgame;


fn main() {
    let dls = rustgame::handle::light::DirectionalLightState::new();
    println!("{}", std::mem::size_of_val(&dls));
}