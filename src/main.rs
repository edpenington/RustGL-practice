#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position : [f32; 3],
    color    : [f32; 3],
}

implement_vertex!(Vertex,position,color);


fn main() {
    use glium::{DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();


    let vertex1 = Vertex { position: [-0.5, -0.5, 0.0],
                           color: [1.0, 0.0, 0.0]};
    let vertex2 = Vertex { position: [ 0.0,  0.5, 0.0],
                           color: [0.0, 1.0, 0.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25, 0.0] ,
                           color: [0.0, 0.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 330

        in vec3 position;
        in vec3 color;
        out vec3 attr;

        uniform mat4 matrix;

        void main() {
            attr = color;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330
        in vec3 attr;
        out vec4 color;
        void main() {
            color = vec4(attr, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = -0.5;
    let mut r: f32 = 0.0;
    loop {
        //Update time-step t
        t += 0.0002;
        r += 0.0002;
        if t > 1.0 {
            t = -1.0;
        }
        let uniforms = uniform! {
            matrix: [
                [r.cos(), r.sin(), 0.0, 0.0],
                [-r.sin(), r.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.7,1.0,0.2,1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
