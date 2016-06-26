extern crate glium;

use maths::vector::Vector3 as Vector3;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position : [f32; 3],
    pub color    : [f32; 3],
}

impl Vertex {
    pub fn from_vector3(position: &Vector3, color: &Vector3) -> Vertex
    {
        let pos_array = [position.x, position.y, position.z];
        let col_array = [color.x, color.y, color.z];
        Vertex {position : pos_array, color: col_array}
    }
}

#[derive(Copy, Clone)]
struct Triangle {
    vertices : [Vertex;3]
}

impl Triangle {
    pub fn from_vertices(v1: Vertex, v2: Vertex, v3: Vertex)
    -> Triangle
    {
        let vertices = [v1,v2,v3];
        Triangle {vertices : vertices}
    }
}

struct Shape {
    triangles : Vec<Triangle>
}

impl Shape {

    pub fn empty() -> Shape
    {
        let empty_triangles : Vec<Triangle> = Vec::new();
        Shape{triangles : empty_triangles}
    }

    pub fn add_triangle(&mut self, tri: Triangle)
    {
        self.triangles.push(tri)
    }

    pub fn as_vert_array(&self) -> Vec<Vertex> {
        let mut vert_array: Vec<Vertex> = Vec::new();
        for tri in &self.triangles {
            vert_array.push(tri.vertices[0]);
            vert_array.push(tri.vertices[1]);
            vert_array.push(tri.vertices[2]);
        }
        vert_array
    }

    pub fn indices(&self) -> Vec<u8>
    {
        let mut indices : Vec<u8> = Vec::new();
        for i in 0..self.triangles.len()
        {
            let mut j  = i as u8;
            j = j - 1;
            indices.push(j);
        }
        indices
    }
}

struct View {
    position: [f32;3],
    direction: [f32;3],
    up: [f32; 3],
}

impl View {
    pub fn vector_length(vec3: &[f32;3]) -> f32 {
        let sum_sqr_vec = (vec3[0] * vec3[0]) +
            (vec3[1] * vec3[1]) +
            (vec3[2] * vec3[2]);
        sum_sqr_vec.sqrt()
    }

    pub fn change_position(&mut self, dx: f32, dy: f32, dz: f32)
    {
        self.position = [
            self.position[0] + dx,
            self.position[1] + dy,
            self.position[2] + dz
        ];
    }

    pub fn as_matrix(&self) ->
        [[f32;4];4] {
            let f = {
                let f = self.direction;
                let len = View::vector_length(&f);
                [f[0] / len, f[1] / len, f[2] / len]
            };
            let s = [self.up[1] * f[2] - self.up[2] * f[1],
                     self.up[2] * f[0] - self.up[0] * f[2],
                     self.up[0] * f[1] - self.up[1] * f[0]];
            let s_norm = {
                let len = View::vector_length(&s);
                [s[0] / len, s[1] / len, s[2] / len]
            };

            let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                     f[2] * s_norm[0] - f[0] * s_norm[2],
                     f[0] * s_norm[1] - f[1] * s_norm[0]];

            let position = self.position;
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
}

/*fn view_matrix(
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
    }*/


pub fn draw_rotating_cube() {
    implement_vertex!(Vertex,position,color);
    use glium::DisplayBuild;
    use glium::Surface;
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
/*

    let vertices = vec![v1, v2, v3, v4, v5, v6, v7, v8];
    let indices_src = vec![0u8,1u8,2u8,
                            2u8,3u8,0u8,
                            0u8,1u8,5u8,
                            5u8,4u8,0u8,
                            4u8,5u8,6u8,
                            6u8,7u8,4u8,
                            3u8,2u8,7u8,
                            7u8,6u8,2u8,
                            0u8,4u8,7u8,
                            7u8,3u8,0u8,
                            1u8,5u8,6u8,
                            6u8,2u8,1u8
                            ];
*/

    let t1 = Triangle::from_vertices(v1,v2,v3);
    let t2 = Triangle::from_vertices(v3,v4,v1);
    let t3 = Triangle::from_vertices(v1,v2,v6);
    let t4 = Triangle::from_vertices(v6,v5,v1);
    let t5 = Triangle::from_vertices(v5,v6,v7);
    let t6 = Triangle::from_vertices(v7,v8,v5);


    let mut shape = Shape::empty();
    shape.add_triangle(t1);
    shape.add_triangle(t2);
    shape.add_triangle(t3);
    shape.add_triangle(t4);
    shape.add_triangle(t5);
    shape.add_triangle(t6);

    let vertices : Vec<Vertex>
        = shape.as_vert_array();
    /*let indices_src : Vec<u8>
        = shape.indices();*/

    let vertex_buffer =
        glium::VertexBuffer::new(
            &display,
            &vertices).unwrap();
    /*let indices =
        glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices_src).unwrap();*/
    let indices =
        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_simple_src = r#"
        #version 330

        in vec3 position;
        in vec3 color;

        uniform mat4 view;

        out vec3 attr;

        void main() {
            attr = color;
            gl_Position = view * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_simple_src = r#"
        #version 330

        in vec3 attr;

        out vec4 color;

        void main() {
            color = vec4(attr, 1.0);
        }

    "#;

    /*let vertex_shader_src = r#"
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
    "#;*/

    /*let fragment_shader_src = r#"
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
    "#;*/

    let program =
        glium::Program::from_source(
            &display,
            vertex_shader_simple_src,
            fragment_shader_simple_src,
            None).unwrap();


    //let mut t: f32 = -0.5;
    //let mut r: f32 = 0.0;

    let mut player_view = View {
        position: [2.0, 2.0, -5.0],
        direction:[0.0, 0.0, 0.0],
        up: [0.0, 1.0, 0.0]
        };

    loop {

        //Update time-step t
        //t += 0.0002;
        //r += 0.0002;
        /*if t > 1.0 {
            t = -1.0;
        }*/

        let mut target = display.draw();

        //let light = [-1.0, 0.4, 0.9f32];
        /*let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];*/


        /*let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };*/

        //player_view.change_position(0.0, 0.0, 0.0);
        let view = player_view.as_matrix();
        /*let perspective = {
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
        };*/

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        /*target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{model: model,
                        view: view,
                        perspective: perspective,
                        u_light: light},
                    &params).unwrap();*/
        let params = Default::default();
        let uniforms = uniform!{view : view};
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
