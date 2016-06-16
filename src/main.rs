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

fn view_matrix(
    position: &[f32;3],
    direction: &[f32;3],
    up: &[f32;3])
    -> [[f32;4];4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
                 let s_norm = {
                     let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
                     let len = len.sqrt();
                     [s[0] / len, s[1] / len, s[2] / len]
                 };

                 let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                          f[2] * s_norm[0] - f[0] * s_norm[2],
                          f[0] * s_norm[1] - f[1] * s_norm[0]];

                 let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                          -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                          -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

                 [
                     [s[0], u[0], f[0], 0.0],
                     [s[1], u[1], f[1], 0.0],
                     [s[2], u[2], f[2], 0.0],
                     [p[0], p[1], p[2], 1.0],
                 ]
    }

fn main() {
    use glium::{DisplayBuild};
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();

    let v1 = Vertex { position: [-0.5, -0.5, 0.5],
                           color: [1.0, 0.0, 0.0]};
    let v2 = Vertex { position: [-0.5, 0.5, 0.5],
                           color: [0.0, 1.0, 0.0] };
    let v3 = Vertex { position: [ 0.5, 0.5, 0.5] ,
                           color: [0.0, 0.0, 1.0] };
    let v4 = Vertex { position: [ 0.5,-0.5, 0.5] ,
                           color: [1.0, 1.0, 1.0] };
    let v5 = Vertex { position: [-0.5, -0.5,-0.5],
                           color: [1.0, 0.0, 0.0]};
    let v6 = Vertex { position: [-0.5, 0.5,-0.5],
                           color: [0.0, 1.0, 0.0] };
    let v7 = Vertex { position: [ 0.5, 0.5,-0.5] ,
                           color: [0.0, 0.0, 1.0] };
    let v8 = Vertex { position: [ 0.5,-0.5,-0.5] ,
                           color: [0.0, 0.0, 0.0] };


    let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
    let indices_src = vec![0u8,1u8,2u8,
                            2u8,3u8,0u8,
                            0u8,1u8,5u8,
                            5u8,4u8,0u8,
                            4u8,5u8,6u8,
                            6u8,7u8,4u8,
                            3u8,2u8,7u8,
                            7u8,6u8,2u8];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices_src).unwrap();

    let vertex_shader_src = r#"
        #version 330

        in vec3 position;
        in vec3 color;

        out vec3 attr;
        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            vec3 normal = vec3(1.0,1.0,1.0);
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            attr = color;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330
        in vec3 attr;
        in vec3 v_normal;
        out vec4 color;

        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.1, 0.1, 0.1);
            color = vec4(mix(dark_color,attr,brightness), 1.0);
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


        let mut target = display.draw();

        let light = [-1.0, 0.4, 0.9f32];
        let model = [
            [r.cos(), r.sin(), 0.0, 0.0],
            [-r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ t , 0.0, 0.0, 1.0f32],
        ];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        let view = view_matrix(&[2.0, -1.0, 1.0*t],
             &[-2.0, 1.0, 1.0],
             &[0.0, 1.0, 0.0]);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{model: model,
                        view: view,
                        perspective: perspective,
                        u_light: light},
                    &params).unwrap();
        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
