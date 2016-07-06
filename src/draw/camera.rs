

use draw::maths::vector::Vector3 as Vector3;

type Matrix = [[f32;4];4];

pub struct Camera {
    position : Vector3,
    direction : Vector3,
    up : Vector3,
}


/*
impl Camera {
    pub fn view_matrix(&self) -> Matrix
    {
        let f = {
            let f = self.direction;
            let len = Vector3::vector_length(&f);
            [f[0] / len, f[1] / len, f[2] / len]
        };
        let s = [self.up[1] * f[2] - self.up[2] * f[1],
                 self.up[2] * f[0] - self.up[0] * f[2],
                 self.up[0] * f[1] - self.up[1] * f[0]];
        let s_norm = {
            let len = Vector3::vector_length(&s);
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
*/
