#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;
mod draw;

fn main() {

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.2,0.2,0.2,1.0),1.0);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
