#[macro_use]
extern crate glium;

//use glium::DisplayBuild;
//use glium::Surface;

mod maths;
mod drawing;

use drawing::draw_rotating_cube;




fn main() {

    draw_rotating_cube();
}
